use std::ops::Not;

use crate::probability::ProbabilityOutcome;

impl Not for ProbabilityOutcome {
    type Output = Self;

    fn not(self) -> Self {
        Self {
            value: !self.value,
            constraint_map: self.constraint_map,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::probability::ProbabilityOutcome;

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_not(value_one: i32) {
            let expected_value = !value_one;
            let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(value_one);
            let result = !probability_outcome;
            assert_eq!(result.value, expected_value);
        }
    }
}
