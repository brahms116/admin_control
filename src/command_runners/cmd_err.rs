#[derive(Debug)]
pub enum CmdErr {
    InvdPwd,
}

impl std::fmt::Display for CmdErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
