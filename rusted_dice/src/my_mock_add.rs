pub fn mock_add(first : i32, second : i32) -> i32 {
    first + second
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_should_fail() {
        let result = mock_add(2, 1);
        assert_eq!(result, 4);
    }

    #[test]
    fn this_test_should_pass() {
        let result = mock_add(12, 92);
        assert_eq!(result, 104);
    }
}
