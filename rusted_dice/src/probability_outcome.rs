#[allow(dead_code)]
struct ProbabilityOutcome {
    value: i64,
    count: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    proptest! {
        #[test]
        fn value_set(test_value: i64) {
            let result = ProbabilityOutcome {value: test_value, count: 0};
            assert!(result.value == test_value);
        }

        #[test]
        fn count_set(test_count: u64) {
            let result = ProbabilityOutcome {value: 0, count: test_count};
            assert!(result.count == test_count);
        }
    }
}