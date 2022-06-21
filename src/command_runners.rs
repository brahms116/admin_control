use crate::core::*;
use async_trait::async_trait;

pub struct CmdRunner<TAuth, TEc2>
where
    TAuth: Auth + Send + Sync,
    TEc2: Ec2Ctrl + Send + Sync,
{
    auth: TAuth,
    ec2: TEc2,
}

impl<A, E> CmdRunner<A, E>
where
    A: Auth + Send + Sync,
    E: Ec2Ctrl + Send + Sync,
{
    pub fn new(auth_service: A, ec2_service: E) -> CmdRunner<A, E> {
        CmdRunner {
            auth: auth_service,
            ec2: ec2_service,
        }
    }
}

#[async_trait]
impl<A, E> Cmd for CmdRunner<A, E>
where
    A: Auth + Send + Sync,
    E: Ec2Ctrl + Send + Sync,
{
    async fn check_token(&self, token: &str) -> bool {
        self.auth.validate_token(token).await
    }
    async fn get_token(&self, pwd: &str) -> Result<String, AdminErr> {
        self.auth.get_token(pwd).await
    }

    async fn ec2_control(&self, op: &Ec2Op) -> Result<Ec2CtrlRes, AdminErr> {
        match op {
            Ec2Op::On => self.ec2.on().await,
            Ec2Op::Off => self.ec2.off().await,
            Ec2Op::Status => self.ec2.status().await,
            _ => Err(AdminErr::InvdEc2Op("Invalid".to_string())),
        }
    }
}
