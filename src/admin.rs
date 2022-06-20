use aws_sdk_ec2::Client;
use serde::Serialize;
use serde_json::Value;

mod admin_errors;
mod router;

pub use admin_errors::*;
pub use router::*;

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

#[derive(Serialize)]
pub enum Ec2Status {
    On,
    Off,
    Pending,
}

pub enum Ec2Op {
    On,
    Off,
    Status,
    Unknown,
}

impl From<&str> for Ec2Op {
    fn from(s: &str) -> Self {
        match s {
            "on" => Ec2Op::On,
            "off" => Ec2Op::Off,
            "status" => Ec2Op::Status,
            _ => Ec2Op::Unknown,
        }
    }
}

#[derive(Serialize)]
pub struct Ec2StatusRes {
    status: Ec2Status,
    ip: Option<String>,
}

pub trait RunCommands {
    fn get_token(
        &self,
        pwd: &str,
        config_pwd: &str,
        jwt_secret: &str,
    ) -> Result<String, AdminErr>;
    fn check_token(&self, token: &str, secret: &str) -> bool;
    fn ec2_control(
        &self,
        instance_id: &str,
        op: &Ec2Op,
    ) -> Result<Ec2StatusRes, AdminErr>;
}

#[cfg(test)]
mod tests {
    use super::*;
    struct MockRunner();

    impl RunCommands for MockRunner {
        fn get_token(
            &self,
            _pwd: &str,
            _config_pwd: &str,
            _jwt_secret: &str,
        ) -> Result<String, AdminErr> {
            Ok("12345".into())
        }

        fn check_token(&self, _token: &str, _secret: &str) -> bool {
            true
        }

        fn ec2_control(
            &self,
            _instance_id: &str,
            _op: &Ec2Op,
        ) -> Result<Ec2StatusRes, AdminErr> {
            Ok(Ec2StatusRes {
                status: Ec2Status::On,
                ip: Some("12.3.45.3".into()),
            })
        }
    }

    #[test]
    fn jwt_env_error() {
        let args = RouteArgs {
            token: None,
            command: None,
            data: None,
        };
        let res = admin(args, MockRunner());
        assert!(res.is_err());
        let res = res.unwrap_err();
        assert_eq!(
            format!("{}", res),
            format!("{}", AdminErr::ConfNone("JWT_SECRET".to_string()))
        );
    }
}
