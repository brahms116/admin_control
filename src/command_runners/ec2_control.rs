use serde_json::Value;

pub enum Operation {
    Status,
    On,
    Off,
    Invalid,
}

impl From<&str> for Operation {
    fn from(op: &str) -> Self {
        match op {
            "status" => Operation::Status,
            "on" => Operation::On,
            "off" => Operation::Off,
            _ => Operation::Invalid,
        }
    }
}

impl From<&String> for Operation {
    fn from(op: &String) -> Self {
        op.into()
    }
}

pub struct ControlParams {
    pub operation: Operation,
    pub instance_id: String,
}

pub fn ec2_control() -> Result<Value, ()> {
    todo!()
}
