#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use common::domain::non_empty_vec::NonEmptyVec;

    #[test]
    fn unable_to_create_empty_vector() {
        let a: Vec<i32> = Vec::new();
        let res = NonEmptyVec::parse(a);
        assert_that!(res).is_err();
    }

    #[test]
    fn should_return_same_vector_for_non_empty() {
        let a = vec![1, 2, 3, 4, 5];
        let non_empty = NonEmptyVec::parse(a.clone()).unwrap();
        let b = Vec::from(non_empty);
        let matching = a.iter().zip(&b).filter(|&(a, b)| a == b).count();
        // All element should match
        assert_eq!(a.len(), matching);
    }
}
