use serde_json::{json, Value};

pub struct HttpError {
    pub status_code: i32,
    pub msg: Option<String>,
}

impl Into<Value> for HttpError {
    fn into(self) -> Value {
        let msg = match self.msg {
            Some(description) => json!(description),
            None => Value::Null,
        };
        lambda_response(json!({ "msg": msg }), self.status_code)
    }
}

pub fn lambda_response(body: Value, code: i32) -> Value {
    return json!({
        "statusCode":code,
        "headers":{
            "Content-Type":"application/json"
        },
        "body":body.to_string()
    });
}

pub fn command_from_req(req: &Value) -> Option<String> {
    let mut body = req.get("body");
    if let Some(body) = body.take() {
        if let Some(command) = body.get("command") {
            if let Some(command) = command.as_str() {
                return Some(command.to_owned());
            }
        }
    }
    None
}

pub fn token_from_req(req: &Value) -> Option<String> {
    let mut headers = req.get("headers");
    if let Some(headers) = headers.take() {
        if let Some(auth) = headers.get("Authorization").take() {
            if auth.is_string() {
                return Some(auth.as_str().unwrap().to_owned());
            }
        }
    }
    None
}
pub fn data_from_req(req: &Value) -> Option<Value> {
    if let Some(body) = req.get("body").take() {
        return body.get("data").map(|e| e.clone());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Person {
        first_name: String,
        last_name: String,
        age: i32,
    }

    #[test]
    fn should_give_lambda_res() {
        let person = Person {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            age: 9,
        };
        let response = lambda_response(json!(person), 200);
        let body = json!({
            "firstName":"John",
            "lastName":"Doe",
            "age":9
        })
        .to_string();

        assert_eq!(
            json!(response),
            json!({
                "statusCode":200,
                "headers":{
                    "Content-Type":"application/json"
                },
                "body":body
            })
        )
    }

    #[test]
    fn should_extract_command_from_req() {
        let correct = json!({
            "body":{
                "command":"ec2-control"
            }
        });
        let command = command_from_req(&correct).unwrap();
        assert_eq!(command, "ec2-control");
    }

    #[test]
    fn should_extract_data_from_req() {
        let req = json!({
            "body":{
                "data":"hello"
            }
        });
        let data = data_from_req(&req).unwrap();
        let string = data.as_str().unwrap();
        assert_eq!("hello", string)
    }

    #[test]
    fn should_extract_token_req() {
        let req = json!({
            "headers":{
                "Authorization":"tokenaswhatfeel"
            }
        });

        let token = token_from_req(&req).unwrap();
        assert_eq!("tokenaswhatfeel".to_string(), token);

        let req = json!({});
        let token = token_from_req(&req);
        assert_eq!(None, token);
    }
}
