use super::*;
use crate::errors::*;
use serde_json::{json, Value};
use std::env;

pub fn admin<T: RunCommands>(
    args: RouteArgs,
    cmd: T,
) -> Result<Value, AdminErr> {
    /* Check for envs */
    let env_jwt_secret = env::var("JWT_SECRET")
        .map_err(|_| AdminErr::ConfNone("JWT_SECRET".to_string()))?;

    let env_pwd = env::var("PASSWORD")
        .map_err(|_| AdminErr::ConfNone("PWD".to_string()))?;

    let ec2_id = env::var("EC2_ID")
        .map_err(|_| AdminErr::ConfNone("EC2_ID".to_string()))?;

    /* Check for valid command */
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
        let result = cmd.check_token(&token, &env_jwt_secret);
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
                if let Some(pwd) = data.get("password") {
                    if let Some(pwd_str) = pwd.as_str() {
                        let token = cmd
                            .get_token(pwd_str, &env_pwd, &env_jwt_secret)
                            .map_err(|_| AdminErr::InvdCreds)?;
                        return Ok(json!({ "token": token }));
                    }
                    return Err(AdminErr::TypeErr(TypeErr {
                        fieldname: "password".into(),
                        expected_type: "string".into(),
                        recieved_type: "unknown".into(),
                    }));
                }
            }
            Err(AdminErr::DataNone("password".into()))
        }
        Command::Ec2Control => {
            if let Some(data) = args.data {
                if let Some(operation) = data.get("operation") {
                    if let Some(op_str) = operation.as_str() {
                        let op: Ec2Op = op_str.clone().into();
                        if let Ec2Op::Unknown = op {
                            return Err(AdminErr::InvdEc2Op(
                                op_str.to_string(),
                            ));
                        }
                        let res = cmd.ec2_control(&ec2_id, &op);
                        return res.map(|v| json!(v));
                    }
                    return Err(AdminErr::TypeErr(TypeErr {
                        fieldname: "operation".into(),
                        expected_type: "string".into(),
                        recieved_type: "unknown".into(),
                    }));
                }
            }
            return Err(AdminErr::DataNone("operation".into()));
        }
        _ => return Err(AdminErr::Unknown),
    }
}
