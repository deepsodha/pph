use anyhow::Result;
use common::{
    domain::{address::Address, contact::Contact, goverment::GovInfo, user::User},
    dto::user::UserDetails,
};

#[derive(Debug, Clone)]
pub struct Staff {
    pub user: User,
    pub address: Address,
    pub phone: Contact,
    pub government_info: GovInfo,
}

impl Staff {
    pub fn parse(a: &UserDetails) -> Result<Staff> {
        let res = Staff {
            user: User::parse(&a.user)?,
            address: Address::parse(&a.address)?,
            phone: Contact::parse(&a.phone)?,
            government_info: GovInfo::parse(&a.government_info)?,
        };
        Ok(res)
    }
}
