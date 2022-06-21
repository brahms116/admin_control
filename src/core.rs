//! Core traits and structs which different part of the App shares
//!
//! It contains the 'interfaces' between the different modules of the app as well as the data
//! structures passed within.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

#[derive(Debug, PartialEq)]
/// Type of commands which can be run
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

#[derive(Debug)]
/// A container for formatting a Type Err
pub struct TypeErr {
    pub fieldname: String,
    pub recieved_type: String,
    pub expected_type: String,
}

impl fmt::Display for TypeErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"For the field {}, we were expecting type {}, but found {} instead.\
        Try using the correct type", self.fieldname, self.expected_type, self.recieved_type)
    }
}

impl std::error::Error for TypeErr {}

#[derive(Debug)]
/// All possible errors within the application
pub enum AdminErr {
    ConfNone(String),
    DataNone,
    InvdData(String),
    TypeErr(TypeErr),
    InvdCmd(String),
    InvdEc2Op(String),
    CmdNone,
    InvdToken,
    Ec2None,
    TokenNone,
    InvdCreds,
    Unknown,
}

impl std::fmt::Display for AdminErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::ConfNone(env) => format!("{} was not configured", env),
            Self::DataNone => {
                format!(
                    "The data object is missing, try adding it to the request"
                )
            }
            Self::InvdData(msg) => msg.to_string(),
            Self::TypeErr(err) => {
                format!("{}", err)
            }
            Self::InvdCmd(cmd) => format!(
                "The given command {} \
                is invalid, please enter a valid command",
                cmd
            ),
            Self::InvdToken => "The auth token is invalid".to_string(),
            Self::TokenNone => "The auth token is missing".to_string(),
            Self::CmdNone => "The command field is missing,\
                                  add an valid command to it"
                .to_string(),
            Self::InvdCreds => {
                "The provided credientials are invalid".to_string()
            }
            _ => format!("Unknown Error"),
        };
        write!(f, "{}", msg)
    }
}

impl std::error::Error for AdminErr {}

#[derive(Deserialize)]
/// json DS for get token
pub struct GetTokenData {
    pub password: String,
}

#[derive(Deserialize)]
/// json DS for ec2 control
pub struct Ec2ControlData {
    pub operation: String,
}

/// Arguments provided to the router to perform routing
pub struct RouteArgs {
    pub token: Option<String>,
    pub command: Option<String>,
    pub data: Option<Value>,
}

/// Can provide our routing functionality
#[async_trait]
pub trait Route {
    async fn route(&self, args: RouteArgs) -> Result<Value, AdminErr>;
}

/// State of an EC2 Instance
#[derive(Serialize)]
pub enum Ec2Status {
    Off,
    On,
    Pending,
}

/// Response from EC2 control
#[derive(Serialize)]
pub struct Ec2CtrlRes {
    status: Ec2Status,
    ip: String,
}

/// Type of operation to be performed by EC2 control
pub enum Ec2Op {
    On,
    Off,
    Status,
    Invalid,
}

impl From<&str> for Ec2Op {
    fn from(s: &str) -> Self {
        match s {
            "on" => Self::On,
            "off" => Self::Off,
            "status" => Self::Status,
            _ => Self::Invalid,
        }
    }
}

impl From<String> for Ec2Op {
    fn from(s: String) -> Self {
        s.as_str().into()
    }
}

/// Can provide command functionality
#[async_trait]
pub trait Cmd {
    async fn check_token(&self, token: &str) -> bool;
    async fn get_token(&self, pwd: &str) -> Result<String, AdminErr>;
    async fn ec2_control(&self, op: &Ec2Op) -> Result<Ec2CtrlRes, AdminErr>;
}
