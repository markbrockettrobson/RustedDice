use std::ops::Mul;

use crate::{
    probability::{Combine, ProbabilityOutcome},
    ValueType,
};

impl Mul for ProbabilityOutcome {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        fn _mul(lhs: ValueType, rhs: ValueType) -> ValueType {
            lhs * rhs
        }
        self.combine(other, _mul)
    }
}

impl Mul<ValueType> for ProbabilityOutcome {
    type Output = Self;

    fn mul(self, other: ValueType) -> Self {
        fn _mul(lhs: ValueType, rhs: ValueType) -> ValueType {
            lhs * rhs
        }
        self.combine_value_type(other, _mul)
    }
}

impl Mul<ProbabilityOutcome> for ValueType {
    type Output = ProbabilityOutcome;

    fn mul(self, other: ProbabilityOutcome) -> ProbabilityOutcome {
        fn _mul(lhs: ValueType, rhs: ValueType) -> ValueType {
            lhs * rhs
        }
        other.value_type_combine(self, _mul)
    }
}

#[cfg(test)]
mod tests {
    use crate::{probability::ProbabilityOutcome, SmallValueType, ValueType};

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_mul(value_one: SmallValueType, value_two: SmallValueType) {
            let expected_value = ValueType::from(value_one) * ValueType::from(value_two);
            let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(value_one.into());
            let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(value_two.into());
            let result = probability_outcome_one * probability_outcome_two;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_mul_value_type(value_one: SmallValueType, value_two: SmallValueType) {
            let expected_value = ValueType::from(value_one) * ValueType::from(value_two);
            let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(value_one.into());
            let result = probability_outcome * ValueType::from(value_two);
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_value_type_mul(value_one: SmallValueType, value_two: SmallValueType) {
            let expected_value = ValueType::from(value_one) * ValueType::from(value_two);
            let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(value_two.into());
            let result = ValueType::from(value_one) * probability_outcome;
            assert_eq!(result.value, expected_value);
        }
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_div_by_zero() {
        let probability_outcome_one =
            ProbabilityOutcome::new_with_empty_constraint_map(ValueType::MAX);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(0);
        let _ = probability_outcome_one / probability_outcome_two;
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_div_value_type_by_zero() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(ValueType::MAX);
        let _ = probability_outcome / 0;
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_value_type_div_by_zero() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(0);
        let _ = ValueType::MAX / probability_outcome;
    }
}
