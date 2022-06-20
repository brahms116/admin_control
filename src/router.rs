use crate::admin_error::*;
use crate::command_runners;
use crate::errors::*;
use crate::services::*;
use serde_json::{json, Value};
use std::env;

#[derive(Debug, PartialEq)]
pub enum Command {
    Ec2Control,
    GetToken,
    Invalid,
}

impl From<&str> for Command {
    fn from(command: &str) -> Self {
        match command {
            "ec2-control" => Command::Ec2Control,
            "get-token" => Command::GetToken,
            "validate-token" => Command::GetToken,
            _ => Command::Invalid,
        }
    }
}
impl From<String> for Command {
    fn from(command: String) -> Self {
        command.as_str().into()
    }
}
pub struct RouteArgs {
    pub token: Option<String>,
    pub command: Option<String>,
    pub data: Option<Value>,
}

pub fn route(args: RouteArgs) -> Result<Value, AdminErr> {
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
        return Err(AdminErr::InvalidCmd(command_str));
    }

    /* If theres no token and command requires token */
    if args.token.is_none() && command != Command::GetToken {
        return Err(AdminErr::TokenNone);
    }

    /* Decode token, if there is a token */
    if let Some(token) = args.token {
        auth_service::decode_default_token(&token, &env_jwt_secret)
            .map_err(|_| AdminErr::InvalidToken)?;
    }

    /* Now the command should be authorised */

    /* Later we should refactor these into handlers */
    /* Need to come up with a better solution for validating */
    match command {
        Command::GetToken => {
            if let Some(data) = args.data {
                if let Some(pwd) = data.get("password") {
                    if let Some(pwd_str) = pwd.as_str() {
                        let token = command_runners::get_token(
                            pwd_str,
                            &env_pwd,
                            &env_jwt_secret,
                        )?;
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
                        todo!()
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
