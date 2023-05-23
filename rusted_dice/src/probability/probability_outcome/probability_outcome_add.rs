use std::ops::Add;

use crate::{
    probability::{Combine, ProbabilityOutcome},
    ValueType,
};

impl Add for ProbabilityOutcome {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        fn _add(lhs: ValueType, rhs: ValueType) -> ValueType {
            lhs + rhs
        }
        self.combine(other, _add)
    }
}

impl Add<ValueType> for ProbabilityOutcome {
    type Output = Self;

    fn add(self, other: ValueType) -> Self {
        fn _add(lhs: ValueType, rhs: ValueType) -> ValueType {
            lhs + rhs
        }
        self.combine_value_type(other, _add)
    }
}

impl Add<ProbabilityOutcome> for ValueType {
    type Output = ProbabilityOutcome;

    fn add(self, other: ProbabilityOutcome) -> ProbabilityOutcome {
        fn _add(lhs: ValueType, rhs: ValueType) -> ValueType {
            lhs + rhs
        }
        other.value_type_combine(self, _add)
    }
}

#[cfg(test)]
mod tests {
    use crate::{probability::ProbabilityOutcome, SmallValueType, ValueType};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_add(value_one: SmallValueType, value_two: SmallValueType) {
            let expected_value = ValueType::from(value_one) + ValueType::from(value_two);
            let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(value_one.into());
            let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(value_two.into());
            let result = probability_outcome_one + probability_outcome_two;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_add_value_type(value_one: SmallValueType, value_two: SmallValueType) {
            let expected_value = ValueType::from(value_one) + ValueType::from(value_two);
            let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(value_one.into());
            let result = probability_outcome + ValueType::from(value_two);
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_value_type_add(value_one: SmallValueType, value_two: SmallValueType) {
            let expected_value = ValueType::from(value_one) + ValueType::from(value_two);
            let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(value_one.into());
            let result = ValueType::from(value_two) + probability_outcome ;
            assert_eq!(result.value, expected_value);
        }
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_add_overflow() {
        let probability_outcome_one =
            ProbabilityOutcome::new_with_empty_constraint_map(ValueType::MAX);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(1);
        let _ = probability_outcome_one + probability_outcome_two;
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_add_underflow() {
        let probability_outcome_one =
            ProbabilityOutcome::new_with_empty_constraint_map(ValueType::MIN);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(-1);
        let _ = probability_outcome_one + probability_outcome_two;
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_add_value_type_overflow() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(ValueType::MAX);
        let _ = probability_outcome + 1;
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_add_value_type_underflow() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(ValueType::MIN);
        let _ = probability_outcome + -1;
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_value_type_add_overflow() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(ValueType::MAX);
        let _ = 1 + probability_outcome;
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_value_type_add_underflow() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(ValueType::MIN);
        let _ = -1 + probability_outcome;
    }
}
