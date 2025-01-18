use anyhow::{bail, Result};
use biscuit_auth::Biscuit;

pub fn get_user_detail(biscuit: &Biscuit) -> Result<(String, String)> {
    let mut authorizer = biscuit.authorizer()?;
    let res: Vec<(String, String)> =
        authorizer.query_all("data($id, $name) <- user($id, $name)")?;
    let user = res.first();
    match user {
        Some(u) => Ok(u.to_owned()),
        None => bail!("Not found"),
    }
}

pub fn get_organization_detail(biscuit: &Biscuit) -> Result<(String, String)> {
    let mut authorizer = biscuit.authorizer()?;
    let res: Vec<(String, String)> =
        authorizer.query_all("data($id, $name) <- organization($id, $name)")?;
    let user = res.first();
    match user {
        Some(u) => Ok(u.to_owned()),
        None => bail!("Not found"),
    }
}

pub fn get_service_location_details(biscuit: &Biscuit) -> Result<(String, String)> {
    let mut authorizer = biscuit.authorizer()?;
    let res: Vec<(String, String)> =
        authorizer.query_all("data($id, $name) <- service_location($id, $name)")?;
    let user = res.first();
    match user {
        Some(u) => Ok(u.to_owned()),
        None => bail!("Not found"),
    }
}

pub fn get_roles(biscuit: &Biscuit) -> Result<Vec<String>> {
    let mut authorizer = biscuit.authorizer()?;
    let res: Vec<(String, bool)> = authorizer.query_all("r($name, true) <- role($name)")?;
    let names: Vec<String> = res
        .into_iter()
        .filter(|(_, role)| *role)
        .map(|(name, _)| name)
        .collect();

    Ok(names)
}
