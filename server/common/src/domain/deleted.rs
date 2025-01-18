#[derive(Debug, Clone, Default)]
pub struct Deleted(pub bool);
impl AsRef<bool> for Deleted {
    fn as_ref(&self) -> &bool {
        &self.0
    }
}
