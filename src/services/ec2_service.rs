use super::core::*;
use super::ec2_control;
use async_trait::async_trait;
use aws_config;
use aws_sdk_ec2::Client;

impl From<i32> for Ec2Status {
    fn from(n: i32) -> Self {
        match n {
            0 | 32 | 64 => Ec2Status::Pending,
            16 => Ec2Status::On,
            80 => Ec2Status::Off,
            48 => Ec2Status::Terminated,
            _ => Ec2Status::Unknown,
        }
    }
}

impl From<ec2_control::Ec2Status> for Ec2CtrlRes {
    fn from(s: ec2_control::Ec2Status) -> Self {
        Ec2CtrlRes {
            status: s.state.into(),
            ip: s.public_ip,
        }
    }
}

impl From<ec2_control::Ec2ControlError> for AdminErr {
    fn from(e: ec2_control::Ec2ControlError) -> Self {
        match e {
            ec2_control::Ec2ControlError::InstanceNotFound => AdminErr::Ec2None,
            ec2_control::Ec2ControlError::Unknown(s) => AdminErr::Ec2Unknown(s),
        }
    }
}

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
