#[derive(Clone, Debug)]
pub struct Reminder(pub bool);
impl Default for Reminder {
    fn default() -> Self {
        Reminder(true)
    }
}

impl AsRef<bool> for Reminder {
    fn as_ref(&self) -> &bool {
        &self.0
    }
}
