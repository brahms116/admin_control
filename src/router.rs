use crate::core::*;
use async_trait::async_trait;
use serde_json::{from_value, json, Value};
use std::marker::Sync;

pub struct Router<T: Cmd + Sync + Send> {
    cmd: T,
}

impl<T: Cmd + Sync + Send> Router<T> {
    pub fn new(cmd: T) -> Router<T> {
        Router { cmd }
    }
}

#[async_trait]
impl<T: Cmd + Sync + Send> Route for Router<T> {
    async fn route(&self, args: RouteArgs) -> Result<Value, AdminErr> {
        if args.command.is_none() {
            return Err(AdminErr::CmdNone);
        }
        let command_str = args.command.unwrap();
        let command = command_str.clone().into();
        if let Command::Invalid = command {
            return Err(AdminErr::InvdCmd(command_str));
        }

        /* If theres no token and command requires token */
        if args.token.is_none() && command != Command::GetToken {
            return Err(AdminErr::TokenNone);
        }

        /* Decode token, if there is a token */
        if let Some(token) = args.token {
            let result = self.cmd.check_token(&token).await;
            if !result {
                return Err(AdminErr::InvdToken);
            }
        }
        /* Now the command should be authorised */

        /* Later we should refactor these into handlers */
        /* Need to come up with a better solution for validating */
        match command {
            Command::GetToken => {
                if let Some(data) = args.data {
                    let params: GetTokenData = from_value(data)
                        .map_err(|e| AdminErr::InvdData(format!("{}", e)))?;
                    return self
                        .cmd
                        .get_token(&params.password)
                        .await
                        .map(|t| json!({ "token": t }));
                }
                return Err(AdminErr::DataNone);
            }
            Command::Ec2Control => {
                if let Some(data) = args.data {
                    let params: Ec2ControlData = from_value(data)
                        .map_err(|e| AdminErr::InvdData(e.to_string()))?;
                    let op: Ec2Op = params.operation.clone().into();
                    if let Ec2Op::Invalid = op {
                        return Err(AdminErr::InvdEc2Op(params.operation));
                    }
                    return self.cmd.ec2_control(&op).await.map(|v| json!(v));
                }
                return Err(AdminErr::DataNone);
            }
            _ => return Err(AdminErr::Unknown),
        }
    }
}
