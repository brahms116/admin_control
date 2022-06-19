mod admin_error;
mod jwt_auth;
mod lambda_http;
mod router;

use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Response {
    status_code: i32,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(handle);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handle(_event: LambdaEvent<Value>) -> Result<Value, Error> {
    Ok(json!(Response {
        status_code: 200,
        body: "HelloWorld".to_owned()
    }))
}
