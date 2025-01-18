use anyhow::{bail, Result};
#[derive(Debug, Clone)]
pub struct NonEmptyVec<T>(Vec<T>);
impl<T> NonEmptyVec<T> {
    pub fn parse(items: Vec<T>) -> Result<NonEmptyVec<T>> {
        if items.is_empty() {
            bail!("Can't be empty")
        } else {
            Ok(NonEmptyVec(items))
        }
    }
}

impl<T> From<NonEmptyVec<T>> for Vec<T> {
    fn from(v: NonEmptyVec<T>) -> Self {
        v.0
    }
}
