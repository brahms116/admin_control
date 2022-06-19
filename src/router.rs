use crate::admin_error::*;
use serde_json::Value;

#[derive(Debug, PartialEq)]
pub enum Command {
    Ec2Control,
    Token,
    Invalid,
}

impl From<&str> for Command {
    fn from(command: &str) -> Self {
        match command {
            "ec2-control" => Command::Ec2Control,
            "token" => Command::Token,
            _ => Command::Invalid,
        }
    }
}

pub struct RouteArgs {
    pub token: Option<String>,
    pub command: Option<String>,
    pub body: Option<Value>,
}

pub fn route(args: RouteArgs) -> Result<Value, AdminError> {
    todo!()
}
