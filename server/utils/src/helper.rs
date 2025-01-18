use anyhow::Result;
use biscuit_auth::builder::{date, fact, string};
use biscuit_auth::macros::authorizer;
use biscuit_auth::{error, Biscuit, KeyPair};
use std::time::{Duration, SystemTime};

pub fn create_token(
    root: &KeyPair,
    user_roles: Vec<&str>,
    user_id: &str,
    org_id: &str,
    org_name: &str,
    user_name: &str,
) -> Result<String> {
    let mut biscuit = Biscuit::builder();
    user_roles.iter().for_each(|&role| {
        biscuit
            .add_fact(fact("role", &[string(role)]))
            .expect("fact adding failed");
    });
    let exp_time = SystemTime::now() + Duration::from_secs(60 * 60 * 24 * 365);
    biscuit
        .add_fact(fact("time", &[date(&exp_time)]))
        .expect("fact adding failed");
    biscuit
        .add_fact(fact("organization", &[string(org_id), string(org_name)]))
        .expect("fact adding failed");
    biscuit
        .add_fact(fact("user", &[string(user_id), string(user_name)]))
        .expect("fact adding failed");
    let biscuit_token = biscuit.build(root).unwrap();
    Ok(biscuit_token.to_base64().unwrap())
}

pub fn _authorize(role: &str, token: &Biscuit) -> Result<(), error::Token> {
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
