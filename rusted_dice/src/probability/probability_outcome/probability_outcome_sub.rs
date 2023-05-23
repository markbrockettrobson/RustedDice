use std::ops::Sub;

use crate::probability::{Combine, ProbabilityOutcome};

impl Sub for ProbabilityOutcome {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        fn _sub(lhs: i32, rhs: i32) -> i32 {
            lhs - rhs
        }
        self.combine(other, _sub)
    }
}

impl Sub<i32> for ProbabilityOutcome {
    type Output = Self;

    fn sub(self, other: i32) -> Self {
        fn _sub(lhs: i32, rhs: i32) -> i32 {
            lhs - rhs
        }
        self.combinei32(other, _sub)
    }
}

impl Sub<ProbabilityOutcome> for i32 {
    type Output = ProbabilityOutcome;

    fn sub(self, other: ProbabilityOutcome) -> ProbabilityOutcome {
        fn _sub(lhs: i32, rhs: i32) -> i32 {
            lhs - rhs
        }
        other.i32combine(self, _sub)
    }
}

#[cfg(test)]
mod tests {
    use crate::probability::ProbabilityOutcome;

    use proptest::prelude::*;

    proptest! {
         #[test]
        fn test_sub(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) - i32::from(value_two);
            let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(value_one.into());
            let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(value_two.into());
            let result = probability_outcome_one - probability_outcome_two;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_sub_i32(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) - i32::from(value_two);
            let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(value_one.into());
            let result = probability_outcome - i32::from(value_two);
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_i32_sub(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) - i32::from(value_two);
            let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(value_two.into());
            let result = i32::from(value_one) - probability_outcome;
            assert_eq!(result.value, expected_value);
        }
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_sub_overflow() {
        let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(i32::MAX);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(-1);
        let _ = probability_outcome_one - probability_outcome_two;
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_sub_underflow() {
        let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(i32::MIN);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(1);
        let _ = probability_outcome_one - probability_outcome_two;
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_sub_i32_overflow() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(i32::MAX);
        let _ = probability_outcome - -1;
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_sub_i32_underflow() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(i32::MIN);
        let _ = probability_outcome - 1;
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_i32_sub_overflow() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(-1);
        let _ = i32::MAX - probability_outcome;
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_132_sub_underflow() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(1);
        let _ = i32::MIN - probability_outcome;
    }
}
