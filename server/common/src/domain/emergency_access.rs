use anyhow::Result;

#[derive(Debug, Clone, Default)]
pub struct EmergencyAccess(bool);
impl AsRef<bool> for EmergencyAccess {
    fn as_ref(&self) -> &bool {
        &self.0
    }
}

impl From<EmergencyAccess> for bool {
    fn from(e: EmergencyAccess) -> Self {
        e.0
    }
}

impl EmergencyAccess {
    pub fn parse(s: &bool) -> Result<EmergencyAccess> {
        Ok(EmergencyAccess(*s))
    }
}
