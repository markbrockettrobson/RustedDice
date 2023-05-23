use std::ops::Mul;

use crate::probability::{Combine, ProbabilityOutcome};

impl Mul for ProbabilityOutcome {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        fn _mul(lhs: i32, rhs: i32) -> i32 {
            lhs * rhs
        }
        self.combine(other, _mul)
    }
}

impl Mul<i32> for ProbabilityOutcome {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        fn _mul(lhs: i32, rhs: i32) -> i32 {
            lhs * rhs
        }
        self.combinei32(other, _mul)
    }
}

impl Mul<ProbabilityOutcome> for i32 {
    type Output = ProbabilityOutcome;

    fn mul(self, other: ProbabilityOutcome) -> ProbabilityOutcome {
        fn _mul(lhs: i32, rhs: i32) -> i32 {
            lhs * rhs
        }
        other.i32combine(self, _mul)
    }
}

#[cfg(test)]
mod tests {
    use crate::probability::ProbabilityOutcome;

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_mul(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) * i32::from(value_two);
            let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(value_one.into());
            let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(value_two.into());
            let result = probability_outcome_one * probability_outcome_two;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_mul_i32(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) * i32::from(value_two);
            let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(value_one.into());
            let result = probability_outcome * i32::from(value_two);
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_i32_mul(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) * i32::from(value_two);
            let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(value_two.into());
            let result = i32::from(value_one) * probability_outcome;
            assert_eq!(result.value, expected_value);
        }
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_div_by_zero() {
        let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(i32::MAX);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(0);
        let _ = probability_outcome_one / probability_outcome_two;
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_div_i32_by_zero() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(i32::MAX);
        let _ = probability_outcome / 0;
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_i32_div_by_zero() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(0);
        let _ = i32::MAX / probability_outcome;
    }
}
