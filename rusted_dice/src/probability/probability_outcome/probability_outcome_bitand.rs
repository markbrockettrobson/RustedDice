use std::ops::BitAnd;

use crate::{
    probability::{Combine, ProbabilityOutcome},
    ValueType,
};

impl BitAnd for ProbabilityOutcome {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        fn _bitand(lhs: ValueType, rhs: ValueType) -> ValueType {
            lhs & rhs
        }
        self.combine(other, _bitand)
    }
}

impl BitAnd<ValueType> for ProbabilityOutcome {
    type Output = Self;

    fn bitand(self, other: ValueType) -> Self {
        fn _bitand(lhs: ValueType, rhs: ValueType) -> ValueType {
            lhs & rhs
        }
        self.combine_value_type(other, _bitand)
    }
}

impl BitAnd<ProbabilityOutcome> for ValueType {
    type Output = ProbabilityOutcome;

    fn bitand(self, other: ProbabilityOutcome) -> ProbabilityOutcome {
        fn _bitand(lhs: ValueType, rhs: ValueType) -> ValueType {
            lhs & rhs
        }
        other.value_type_combine(self, _bitand)
    }
}

#[cfg(test)]
mod tests {
    use crate::{probability::ProbabilityOutcome, SmallValueType, ValueType};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_bitand(value_one: ValueType, value_two: ValueType) {
            let expected_value = value_one & value_two;
            let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(value_one);
            let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(value_two);
            let result = probability_outcome_one & probability_outcome_two;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_bitand_value_type(value_one: SmallValueType, value_two: SmallValueType) {
            let expected_value = ValueType::from(value_one) & ValueType::from(value_two);
            let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(value_one.into());
            let result = probability_outcome & ValueType::from(value_two);
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_value_type_bitand(value_one: SmallValueType, value_two: SmallValueType) {
            let expected_value = ValueType::from(value_one) & ValueType::from(value_two);
            let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(value_two.into());
            let result = ValueType::from(value_one) & probability_outcome;
            assert_eq!(result.value, expected_value);
        }
    }
}
