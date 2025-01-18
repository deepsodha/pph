use anyhow::{bail, Result};

#[derive(Debug, Clone)]
pub struct RequiredMax300Words(String);
impl RequiredMax300Words {
    pub fn parse(cm: &str) -> Result<RequiredMax300Words> {
        // TODO: this will be change
        if cm.len() <= 300 {
            let res = RequiredMax300Words(String::from(cm));
            Ok(res)
        } else {
            // TODO: add proper error using thisError
            bail!("invalid comments")
        }
    }
}

impl AsRef<str> for RequiredMax300Words {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct CommentsInput {
    pub words: String,
}

#[derive(Debug, Clone)]
pub struct Comments {
    pub words: RequiredMax300Words,
}

#[derive(Debug, Clone)]
pub struct Instructions {
    pub words: RequiredMax300Words,
}
