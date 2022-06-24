#[allow(dead_code)]
struct ProbabilityOutcome {
    value: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    proptest! {
        #[test]
        fn value_set(test_value: i64) {
            let result = ProbabilityOutcome {value: test_value};
            assert!(result.value == test_value);
        }
    }
}