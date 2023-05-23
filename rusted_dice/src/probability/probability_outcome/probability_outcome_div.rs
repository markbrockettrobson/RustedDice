use std::ops::Div;

use crate::{
    probability::{Combine, ProbabilityOutcome},
    ValueType,
};

impl Div for ProbabilityOutcome {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        fn _div(lhs: ValueType, rhs: ValueType) -> ValueType {
            lhs / rhs
        }
        self.combine(other, _div)
    }
}

impl Div<ValueType> for ProbabilityOutcome {
    type Output = Self;

    fn div(self, other: ValueType) -> Self {
        fn _div(lhs: ValueType, rhs: ValueType) -> ValueType {
            lhs / rhs
        }
        self.combine_value_type(other, _div)
    }
}

impl Div<ProbabilityOutcome> for ValueType {
    type Output = ProbabilityOutcome;

    fn div(self, other: ProbabilityOutcome) -> ProbabilityOutcome {
        fn _div(lhs: ValueType, rhs: ValueType) -> ValueType {
            lhs / rhs
        }
        other.value_type_combine(self, _div)
    }
}

#[cfg(test)]
mod tests {
    use crate::{probability::ProbabilityOutcome, SmallValueType, ValueType};

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_div(value_one: ValueType, value_two: SmallValueType) {
            prop_assume!(value_two != 0);
            let expected_value = value_one / ValueType::from(value_two);
            let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(value_one);
            let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(value_two.into());
            let result = probability_outcome_one / probability_outcome_two;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_div_value_type(value_one: SmallValueType, value_two: SmallValueType) {
            prop_assume!(value_two != 0);
            let expected_value = ValueType::from(value_one) / ValueType::from(value_two);
            let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(value_one.into());
            let result = probability_outcome / ValueType::from(value_two);
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_value_type_div(value_one: SmallValueType, value_two: SmallValueType) {
            prop_assume!(value_two != 0);
            let expected_value = ValueType::from(value_one) / ValueType::from(value_two);
            let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(value_two.into());
            let result = ValueType::from(value_one) / probability_outcome;
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
