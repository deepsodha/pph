use crate::dto::login_mobile::LoginMobile;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginWithMobileCommand {
    pub mobile: String,
    pub password: String,
}

impl From<LoginMobile> for LoginWithMobileCommand {
    fn from(u: LoginMobile) -> Self {
        LoginWithMobileCommand {
            mobile: String::from(&u.mobile_number),
            password: String::from(&u.password),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LoginCommand {
    LoginWithMobileCommand(LoginWithMobileCommand),
}
