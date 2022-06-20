mod cmd_err;
mod ec2_control;
mod get_token;

use crate::services;
pub use cmd_err::*;
use ec2_control::*;
use get_token::*;

pub enum Ec2Status {
    On,
    Off,
    Pending,
}

pub struct Ec2StatusRes<'a> {
    pub status: Ec2Status,
    pub ip: Option<&'a str>,
}

pub trait Ec2Control {
    fn get_status(&self) -> Ec2StatusRes;
    fn on(&self) -> Ec2StatusRes;
    fn off(&self) -> Ec2StatusRes;
}

pub trait Auth {
    fn get_token(secret: &str) -> String;
    fn check_token(token: &str, secret: &str) -> bool;
}

pub struct CmdRunner<TAuth, TEc2Control>
where
    TAuth: Auth,
    TEc2Control: Ec2Control,
{
    auth: TAuth,
    ec2: TEc2Control,
}

impl<T, E> CmdRunner<T, E>
where
    T: Auth,
    E: Ec2Control,
{
    pub fn new(auth_service: T, ec2_service: E) -> CmdRunner<T, E> {
        CmdRunner {
            auth: auth_service,
            ec2: ec2_service,
        }
    }
}
