use aws_sdk_ec2::Client;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct Ec2Status {
    pub state: i32,
    pub public_ip: Option<String>,
}

#[derive(Debug)]
pub enum Ec2ControlError {
    Unknown(String),
    InstanceNotFound,
}

impl Display for Ec2ControlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Ec2ControlError::Unknown(unknown) => {
                format!("Unknown Error: {}", unknown)
            }
            Ec2ControlError::InstanceNotFound => {
                "Instance Not Found!".to_owned()
            }
        };
        write!(f, "{}", msg)
    }
}

impl Error for Ec2ControlError {}

pub async fn get_ec2_status(
    client: &Client,
    instance_id: &str,
) -> Result<Ec2Status, Ec2ControlError> {
    let descrip = client
        .describe_instances()
        .instance_ids(instance_id)
        .send()
        .await
        .map_err(|e| Ec2ControlError::Unknown(format!("{}", e)))?;
    let instance = descrip
        .reservations()
        .ok_or(Ec2ControlError::InstanceNotFound)?
        .get(0)
        .ok_or(Ec2ControlError::InstanceNotFound)?
        .instances()
        .ok_or(Ec2ControlError::InstanceNotFound)?
        .get(0)
        .ok_or(Ec2ControlError::InstanceNotFound)?;

    let state = instance.state().unwrap().code().unwrap();
    let public_ip = instance.public_ip_address().map(|e| e.to_owned());

    Ok(Ec2Status { state, public_ip })
}

pub async fn start_ec2(
    client: &Client,
    instance_id: &str,
) -> Result<Ec2Status, Ec2ControlError> {
    let res = client
        .start_instances()
        .instance_ids(instance_id)
        .send()
        .await
        .map_err(|e| Ec2ControlError::Unknown(format!("{}", e)))?;
    let instance = res
        .starting_instances()
        .ok_or(Ec2ControlError::InstanceNotFound)?
        .get(0)
        .ok_or(Ec2ControlError::InstanceNotFound)?;
    let code = instance.current_state().unwrap().code().unwrap();
    Ok(Ec2Status {
        state: code,
        public_ip: None,
    })
}

pub async fn stop_ec2(
    client: &Client,
    instance_id: &str,
) -> Result<Ec2Status, Ec2ControlError> {
    let res = client
        .stop_instances()
        .instance_ids(instance_id)
        .send()
        .await
        .map_err(|e| Ec2ControlError::Unknown(format!("{}", e)))?;
    let instance = res
        .stopping_instances()
        .ok_or(Ec2ControlError::InstanceNotFound)?
        .get(0)
        .ok_or(Ec2ControlError::InstanceNotFound)?;
    let code = instance.current_state().unwrap().code().unwrap();
    Ok(Ec2Status {
        state: code,
        public_ip: None,
    })
}
