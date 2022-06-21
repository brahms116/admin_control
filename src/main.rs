mod command_runners;
mod core;
mod lambda_http;
mod lambda_http_impl;
mod router;
mod services;

use lambda_http::*;
use lambda_runtime::{service_fn, LambdaEvent};
use serde_json::Value;

use crate::core::*;
use command_runners::*;
use router::*;
use services::*;

type StdErr = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), StdErr> {
    let func = service_fn(handle);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handle(event: LambdaEvent<Value>) -> Result<Value, StdErr> {
    let (event, _context) = event.into_parts();
    let cmd = command_from_req(&event);
    let data = data_from_req(&event);
    let token = token_from_req(&event);

    /* Setup services */
    let auth = AuthService::new();
    if let Err(e) = auth {
        return Ok(e.into());
    }
    let auth = auth.unwrap();

    let ec2 = Ec2Service::new().await;
    if let Err(e) = ec2 {
        return Ok(e.into());
    }
    let ec2 = ec2.unwrap();
    let cmd_runner = CmdRunner::new(auth, ec2);
    let router = Router::new(cmd_runner);

    let res = router
        .route(RouteArgs {
            token,
            data,
            command: cmd,
        })
        .await;

    if res.is_err() {
        return Ok(res.unwrap_err().into());
    }
    Ok(lambda_response(res.unwrap(), 200))
}
