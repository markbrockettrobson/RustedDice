pub fn mock_add(first : i32, second : i32) -> i32 {
    first + second
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use proptest::prelude::*;

    #[test_case(-2, -4, -6 ; "when both operands are negative")]
    #[test_case(-4,  2, -2 ; "whem 1st operand is negative")]
    #[test_case(2,  -4, -2 ; "when 2nd operand is negative")]
    fn this_test_should_fail(operand_one: i32, operand_two: i32, expected_awnser: i32) {
        let result = mock_add(operand_one, operand_two);
        assert_eq!(result, expected_awnser);
    }

    proptest! {
        #[test]
        fn add_positive_should_be_larger(basevalue: i32, positive: u16) {
            let result = mock_add(basevalue, positive.into());
            assert!(result >= basevalue);
        }
    }
}
