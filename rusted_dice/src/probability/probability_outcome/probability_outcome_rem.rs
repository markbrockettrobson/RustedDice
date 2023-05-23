use std::ops::Rem;

use crate::probability::{Combine, ProbabilityOutcome};

impl Rem for ProbabilityOutcome {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        fn _rem(lhs: i32, rhs: i32) -> i32 {
            lhs % rhs
        }
        self.combine(other, _rem)
    }
}

impl Rem<i32> for ProbabilityOutcome {
    type Output = Self;

    fn rem(self, other: i32) -> Self {
        fn _rem(lhs: i32, rhs: i32) -> i32 {
            lhs % rhs
        }
        self.combinei32(other, _rem)
    }
}

impl Rem<ProbabilityOutcome> for i32 {
    type Output = ProbabilityOutcome;

    fn rem(self, other: ProbabilityOutcome) -> ProbabilityOutcome {
        fn _rem(lhs: i32, rhs: i32) -> i32 {
            lhs % rhs
        }
        other.i32combine(self, _rem)
    }
}

#[cfg(test)]
mod tests {
    use crate::probability::ProbabilityOutcome;

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_rem(value_one: i32, value_two: i16) {
            prop_assume!(value_two != 0);
            let expected_value = value_one % i32::from(value_two);
            let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(value_one);
            let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(value_two.into());
            let result = probability_outcome_one % probability_outcome_two;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_rem_i32(value_one: i16, value_two: i16) {
            prop_assume!(value_two != 0);
            let expected_value = i32::from(value_one) % i32::from(value_two);
            let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(value_one.into());
            let result = probability_outcome % i32::from(value_two);
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_i32_rem(value_one: i16, value_two: i16) {
            prop_assume!(value_two != 0);
            let expected_value = i32::from(value_one) % i32::from(value_two);
            let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(value_two.into());
            let result = i32::from(value_one) % probability_outcome;
            assert_eq!(result.value, expected_value);
        }
    }

    #[test]
    #[should_panic(expected = "attempt to calculate the remainder with a divisor of zero")]
    fn test_rem_by_zero() {
        let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(i32::MAX);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(0);
        let _ = probability_outcome_one % probability_outcome_two;
    }

    #[test]
    #[should_panic(expected = "attempt to calculate the remainder with a divisor of zero")]
    fn test_rem_i32_by_zero() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(i32::MAX);
        let _ = probability_outcome % 0;
    }

    #[test]
    #[should_panic(expected = "attempt to calculate the remainder with a divisor of zero")]
    fn test_i32_rem_by_zero() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(0);
        let _ = i32::MAX % probability_outcome;
    }
}
