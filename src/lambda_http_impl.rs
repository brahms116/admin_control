use crate::core::*;
use crate::lambda_http::*;
use serde_json::Value;

impl From<AdminErr> for HttpError {
    fn from(e: AdminErr) -> Self {
        let code = match e {
            AdminErr::ConfNone(_)
            | AdminErr::Ec2None
            | AdminErr::Unknown
            | AdminErr::Ec2Unknown(_) => 500,
            AdminErr::InvdToken => 403,
            _ => 400,
        };

        HttpError {
            status_code: code,
            msg: Some(format!("{}", e)),
        }
    }
}

impl Into<Value> for AdminErr {
    fn into(self) -> Value {
        let http: HttpError = self.into();
        http.into()
    }
}
