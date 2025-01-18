#[derive(Debug, Clone, Default)]
pub struct Locked(pub bool);
impl AsRef<bool> for Locked {
    fn as_ref(&self) -> &bool {
        &self.0
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn as_ref_test() {
//         let input = Locked::default();
//         let res = input.as_ref();
//         assert_eq!(res, &false);
//     }
// }
