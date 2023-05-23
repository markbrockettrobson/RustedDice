use std::ops::BitAnd;

use crate::probability::{Combine, ProbabilityOutcome};

impl BitAnd for ProbabilityOutcome {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        fn _bitand(lhs: i32, rhs: i32) -> i32 {
            lhs & rhs
        }
        self.combine(other, _bitand)
    }
}

impl BitAnd<i32> for ProbabilityOutcome {
    type Output = Self;

    fn bitand(self, other: i32) -> Self {
        fn _bitand(lhs: i32, rhs: i32) -> i32 {
            lhs & rhs
        }
        self.combinei32(other, _bitand)
    }
}

impl BitAnd<ProbabilityOutcome> for i32 {
    type Output = ProbabilityOutcome;

    fn bitand(self, other: ProbabilityOutcome) -> ProbabilityOutcome {
        fn _bitand(lhs: i32, rhs: i32) -> i32 {
            lhs & rhs
        }
        other.i32combine(self, _bitand)
    }
}

#[cfg(test)]
mod tests {
    use crate::probability::ProbabilityOutcome;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_bitand(value_one: i32, value_two: i32) {
            let expected_value = value_one & value_two;
            let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(value_one);
            let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(value_two);
            let result = probability_outcome_one & probability_outcome_two;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_bitand_i32(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) & i32::from(value_two);
            let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(value_one.into());
            let result = probability_outcome & i32::from(value_two);
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_i32_bitand(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) & i32::from(value_two);
            let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(value_two.into());
            let result = i32::from(value_one) & probability_outcome;
            assert_eq!(result.value, expected_value);
        }
    }
}
