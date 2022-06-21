use super::core::*;
use super::ec2_control;
use async_trait::async_trait;
use aws_config;
use aws_sdk_ec2::Client;

pub struct Ec2Service {
    instance_id: String,
    client: Client,
}

impl Ec2Service {
    pub async fn new() -> Result<Ec2Service, AdminErr> {
        let config = aws_config::load_from_env().await;
        let client = Client::new(&config);
        let id = std::env::var("EC2_ID")
            .map_err(|_| AdminErr::ConfNone("EC2_ID".to_string()))?;
        Ok(Ec2Service {
            instance_id: id,
            client,
        })
    }
}

#[async_trait]
impl Ec2Ctrl for Ec2Service {
    async fn status(&self) -> Result<Ec2CtrlRes, AdminErr> {
        ec2_control::get_ec2_status(&self.client, &self.instance_id)
            .await
            .map(|v| v.into())
            .map_err(|e| e.into())
    }
    async fn on(&self) -> Result<Ec2CtrlRes, AdminErr> {
        ec2_control::start_ec2(&self.client, &self.instance_id)
            .await
            .map(|v| v.into())
            .map_err(|e| e.into())
    }
    async fn off(&self) -> Result<Ec2CtrlRes, AdminErr> {
        ec2_control::stop_ec2(&self.client, &self.instance_id)
            .await
            .map(|v| v.into())
            .map_err(|e| e.into())
    }
}
