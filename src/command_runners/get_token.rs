use super::admin_error::*;
use super::services::*;

pub fn get_token(
    password: &str,
    correct_password: &str,
    jwt_secret: &str,
) -> Result<String, AdminErr> {
    if password != correct_password {
        return Err(AdminErr::InvalidCreds);
    }
    Ok(auth_service::get_default_token(jwt_secret, 3600))
}
