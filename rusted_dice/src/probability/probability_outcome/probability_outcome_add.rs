use std::ops::Add;

use crate::probability::{Combine, ProbabilityOutcome};

impl Add for ProbabilityOutcome {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        fn _add(lhs: i32, rhs: i32) -> i32 {
            lhs + rhs
        }
        self.combine(other, _add)
    }
}

impl Add<i32> for ProbabilityOutcome {
    type Output = Self;

    fn add(self, other: i32) -> Self {
        fn _add(lhs: i32, rhs: i32) -> i32 {
            lhs + rhs
        }
        self.combinei32(other, _add)
    }
}

impl Add<ProbabilityOutcome> for i32 {
    type Output = ProbabilityOutcome;

    fn add(self, other: ProbabilityOutcome) -> ProbabilityOutcome {
        fn _add(lhs: i32, rhs: i32) -> i32 {
            lhs + rhs
        }
        other.i32combine(self, _add)
    }
}

#[cfg(test)]
mod tests {
    use crate::probability::ProbabilityOutcome;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_add(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) + i32::from(value_two);
            let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(value_one.into());
            let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(value_two.into());
            let result = probability_outcome_one + probability_outcome_two;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_add_i32(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) + i32::from(value_two);
            let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(value_one.into());
            let result = probability_outcome + i32::from(value_two);
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_i32_add(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) + i32::from(value_two);
            let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(value_one.into());
            let result = i32::from(value_two) + probability_outcome ;
            assert_eq!(result.value, expected_value);
        }
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_add_overflow() {
        let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(i32::MAX);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(1);
        let _ = probability_outcome_one + probability_outcome_two;
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_add_underflow() {
        let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(i32::MIN);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(-1);
        let _ = probability_outcome_one + probability_outcome_two;
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_add_i32_overflow() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(i32::MAX);
        let _ = probability_outcome + 1;
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_add_i32_underflow() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(i32::MIN);
        let _ = probability_outcome + -1;
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_i32_add_overflow() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(i32::MAX);
        let _ = 1 + probability_outcome;
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_i32_add_underflow() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(i32::MIN);
        let _ = -1 + probability_outcome;
    }
}
