use std::ops::Div;

use crate::probability::{Combine, ProbabilityOutcome};

impl Div for ProbabilityOutcome {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        fn _div(lhs: i32, rhs: i32) -> i32 {
            lhs / rhs
        }
        self.combine(other, _div)
    }
}

impl Div<i32> for ProbabilityOutcome {
    type Output = Self;

    fn div(self, other: i32) -> Self {
        fn _div(lhs: i32, rhs: i32) -> i32 {
            lhs / rhs
        }
        self.combinei32(other, _div)
    }
}

impl Div<ProbabilityOutcome> for i32 {
    type Output = ProbabilityOutcome;

    fn div(self, other: ProbabilityOutcome) -> ProbabilityOutcome {
        fn _div(lhs: i32, rhs: i32) -> i32 {
            lhs / rhs
        }
        other.i32combine(self, _div)
    }
}

#[cfg(test)]
mod tests {
    use crate::probability::ProbabilityOutcome;

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_div(value_one: i32, value_two: i16) {
            prop_assume!(value_two != 0);
            let expected_value = value_one / i32::from(value_two);
            let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(value_one);
            let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(value_two.into());
            let result = probability_outcome_one / probability_outcome_two;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_div_i32(value_one: i16, value_two: i16) {
            prop_assume!(value_two != 0);
            let expected_value = i32::from(value_one) / i32::from(value_two);
            let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(value_one.into());
            let result = probability_outcome / i32::from(value_two);
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_i32_div(value_one: i16, value_two: i16) {
            prop_assume!(value_two != 0);
            let expected_value = i32::from(value_one) / i32::from(value_two);
            let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(value_two.into());
            let result = i32::from(value_one) / probability_outcome;
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
