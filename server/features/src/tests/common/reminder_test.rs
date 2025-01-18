#[cfg(test)]
mod tests {
    use common::domain::reminder::Reminder;

    #[test]
    fn reminder_test() {
        let input = Reminder::default();
        let res = input.as_ref();
        assert_eq!(res, &true);
    }
}
