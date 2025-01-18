use super::required_string::RequiredString;
use crate::dto::address::AddressInput;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct PinCode(RequiredString);
impl PinCode {
    pub fn parse(s: &str) -> Result<PinCode> {
        Ok(PinCode(RequiredString::parse(s)?))
    }
}

impl AsRef<str> for PinCode {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct City(RequiredString);
impl City {
    pub fn parse(s: &str) -> Result<City> {
        Ok(City(RequiredString::parse(s)?))
    }
}

impl AsRef<str> for City {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct State(RequiredString);
impl State {
    pub fn parse(s: &str) -> Result<State> {
        Ok(State(RequiredString::parse(s)?))
    }
}

impl AsRef<str> for State {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct District(RequiredString);
impl District {
    pub fn parse(s: &str) -> Result<District> {
        Ok(District(RequiredString::parse(s)?))
    }
}

impl AsRef<str> for District {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct Country(RequiredString);
impl Country {
    pub fn parse(s: &str) -> Result<Country> {
        Ok(Country(RequiredString::parse(s)?))
    }
}

impl AsRef<str> for Country {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct AddressLine(RequiredString);
impl AddressLine {
    pub fn parse(s: &str) -> Result<AddressLine> {
        Ok(AddressLine(RequiredString::parse(s)?))
    }
}

impl AsRef<str> for AddressLine {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<Address> for AddressInput {
    fn from(s: Address) -> Self {
        AddressInput {
            pin_code: String::from(s.pin_code.as_ref()),
            city: String::from(s.city.as_ref()),
            country: String::from(s.country.as_ref()),
            state: String::from(s.state.as_ref()),
            address_line: String::from(s.address_line.as_ref()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Address {
    pub pin_code: PinCode,
    pub city: City,
    pub state: State,
    pub country: Country,
    pub address_line: AddressLine,
}
impl Address {
    // TODO: May be with `from impl` we can have marriage between this and graphql input
    pub fn parse(input: &AddressInput) -> Result<Address> {
        let res = Address {
            pin_code: PinCode::parse(&input.pin_code)?,
            city: City::parse(&input.city)?,
            country: Country::parse(&input.country)?,
            state: State::parse(&input.state)?,
            address_line: AddressLine::parse(&input.address_line)?,
        };
        Ok(res)
    }
}
