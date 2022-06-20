use crate::errors::*;

#[derive(Debug)]
pub enum AdminErr {
    ConfNone(String),
    DataNone(String),
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
            Self::DataNone(field) => {
                format!(
                    "{} is missing from the data field, please add \
                    it in the data object",
                    field
                )
            }
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