use anyhow::Result;
use biscuit_auth::builder::{date, fact, string};
use biscuit_auth::macros::authorizer;
use biscuit_auth::{error, Biscuit, KeyPair};
use std::time::{Duration, SystemTime};

pub fn create_token(root: &KeyPair, data: TokenData) -> Result<String> {
    let mut biscuit = Biscuit::builder();
    data.user_roles.iter().for_each(|role| {
        biscuit
            .add_fact(fact("role", &[string(role)]))
            .expect("fact adding failed");
    });
    let exp_time = SystemTime::now() + Duration::from_secs(60 * 60 * 24 * 365);
    biscuit
        .add_fact(fact("time", &[date(&exp_time)]))
        .expect("fact adding failed");
    biscuit
        .add_fact(fact(
            "organization",
            &[string(data.org_id), string(data.org_name)],
        ))
        .expect("fact adding failed");
    biscuit
        .add_fact(fact(
            "user",
            &[string(data.user_id), string(data.user_name)],
        ))
        .expect("fact adding failed");
    biscuit
        .add_fact(fact(
            "service_location",
            &[
                string(data.service_location_id),
                string(data.service_location_name),
            ],
        ))
        .expect("fact adding failed");
    let biscuit_token = biscuit.build(root).unwrap();
    Ok(biscuit_token.to_base64().unwrap())
}

pub fn authorize(role: &str, token: &Biscuit) -> Result<(), error::Token> {
    let mut authorizer = authorizer!(
        r#"
         allow if role({role});
         check if time($time), $time > {expiration};
      "#,
        expiration = SystemTime::now()
    );
    // register a fact containing the current time for TTL checks
    authorizer.set_time();

    authorizer.add_token(token)?;

    authorizer.authorize()?;

    Ok(())
}

pub struct TokenData<'a> {
    pub user_roles: Vec<&'a str>,
    pub user_id: &'a str,
    pub org_id: &'a str,
    pub org_name: &'a str,
    pub user_name: &'a str,
    pub service_location_id: &'a str,
    pub service_location_name: &'a str,
}
