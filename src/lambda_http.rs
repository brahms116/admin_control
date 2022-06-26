use serde_json::{from_str, json, Value};

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
            "Content-Type":"application/json",
            "Access-Control-Origin":"*"
        },
        "body":body.to_string()
    });
}

pub fn command_from_req(req: &Value) -> Option<String> {
    let body = req
        .get("body")
        .map(|e| e.as_str())
        .flatten()
        .map(|e| from_str::<Value>(e).ok())
        .flatten();
    if let Some(b) = body {
        if let Some(command) = b.get("command") {
            if let Some(cmd) = command.as_str() {
                return Some(cmd.to_string());
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
    let body = req
        .get("body")
        .map(|e| e.as_str())
        .flatten()
        .map(|e| from_str::<Value>(e).ok())
        .flatten();
    if let Some(body) = body {
        return body.get("data").map(|e| e.clone());
    }
    None
}
