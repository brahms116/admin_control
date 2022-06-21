use super::core::*;
use super::ec2_control;

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
