fn add_numbers(first_num: i32, second_num: i32) -> i32 {
    first_num + second_num
}

#[cfg(test)]
mod my_test_suite {
    use super::*;

    #[test]
    fn successfully_adds_numbers() {
        assert_eq!(4, add_numbers(2, 2));
    }
}