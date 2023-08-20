use std::ops::Mul;

use crate::{
    probability::{Combine, ProbabilityOutcome},
    ValueType,
};

fn _mul(lhs: ValueType, rhs: ValueType) -> ValueType {
    lhs * rhs
}

impl Mul for ProbabilityOutcome {
    type Output = Self;

    /// Implements the multiply operator for [ProbabilityOutcome].
    /// values are combined using the multiply function.
    /// constraint maps are combined using the ConstraintMap::add function.
    ///
    /// # Arguments
    ///
    /// * `self` - The first [ProbabilityOutcome] operand.
    /// * `other` - The second [ProbabilityOutcome] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityOutcome] after the multiply operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::constraint_management::Constraint;
    /// # use crate::rusted_dice::constraint_management::ConstraintMap;
    /// # use crate::rusted_dice::probability::ProbabilityOutcome;
    /// let constraint_map_one = ConstraintMap::new_constraint_map(
    ///     vec![
    ///        Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
    ///        Constraint::new_many_item_constraint(2, vec![1, 2, 3])
    ///     ]
    /// );
    /// let probability_outcome_one = ProbabilityOutcome::new_with_constraint_map(
    ///     100, constraint_map_one
    /// );
    ///
    /// let constraint_map_two = ConstraintMap::new_constraint_map(
    ///     vec![
    ///        Constraint::new_many_item_constraint(1, vec![3, 4, 5])
    ///     ]
    /// );
    /// let probability_outcome_two = ProbabilityOutcome::new_with_constraint_map(
    ///     200, constraint_map_two
    /// );
    ///
    /// let constraint_map_three = ConstraintMap::new_constraint_map(
    ///     vec![
    ///        Constraint::new_many_item_constraint(1, vec![3]),
    ///        Constraint::new_many_item_constraint(2, vec![1, 2, 3])
    ///     ]
    /// );
    /// let probability_outcome_three = ProbabilityOutcome::new_with_constraint_map(
    ///     20000, constraint_map_three
    /// );
    ///
    /// assert_eq!(probability_outcome_one * probability_outcome_two, probability_outcome_three);
    /// ```
    fn mul(self, other: Self) -> Self {
        self.combine(other, _mul)
    }
}

impl Mul<ValueType> for ProbabilityOutcome {
    type Output = Self;

    /// Implements the multiply operator for [ProbabilityOutcome] * [ValueType].
    /// values are combined using the multiply function.
    /// constraint map is taken from the [ProbabilityOutcome].
    ///
    /// # Arguments
    ///
    /// * `self` - The [ProbabilityOutcome] operand.
    /// * `other` - The [ValueType] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityOutcome] after the multiply operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::constraint_management::Constraint;
    /// # use crate::rusted_dice::constraint_management::ConstraintMap;
    /// # use crate::rusted_dice::probability::ProbabilityOutcome;
    /// let constraint_map = ConstraintMap::new_constraint_map(
    ///     vec![
    ///        Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
    ///        Constraint::new_many_item_constraint(2, vec![1, 2, 3])
    ///     ]
    /// );
    /// let probability_outcome_one = ProbabilityOutcome::new_with_constraint_map(
    ///     100, constraint_map.clone()
    /// );
    ///
    /// let probability_outcome_two = ProbabilityOutcome::new_with_constraint_map(
    ///     20000, constraint_map
    /// );
    ///
    /// assert_eq!(probability_outcome_one * 200, probability_outcome_two);
    /// ```
    fn mul(self, other: ValueType) -> Self {
        self.combine_value_type(other, _mul)
    }
}

impl Mul<ProbabilityOutcome> for ValueType {
    type Output = ProbabilityOutcome;

    /// Implements the multiply operator for [ValueType] * [ProbabilityOutcome].
    /// values are combined using the multiply function.
    /// constraint map is taken from the [ProbabilityOutcome].
    ///
    /// # Arguments
    ///
    /// * `self` - The [ValueType] operand.
    /// * `other` - The [ProbabilityOutcome] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityOutcome] after the multiply operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::constraint_management::Constraint;
    /// # use crate::rusted_dice::constraint_management::ConstraintMap;
    /// # use crate::rusted_dice::probability::ProbabilityOutcome;
    /// let constraint_map = ConstraintMap::new_constraint_map(
    ///     vec![
    ///        Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
    ///        Constraint::new_many_item_constraint(2, vec![1, 2, 3])
    ///     ]
    /// );
    /// let probability_outcome_one = ProbabilityOutcome::new_with_constraint_map(
    ///     100, constraint_map.clone()
    /// );
    ///
    /// let probability_outcome_two = ProbabilityOutcome::new_with_constraint_map(
    ///     20000, constraint_map
    /// );
    ///
    /// assert_eq!(200 * probability_outcome_one, probability_outcome_two);
    /// ```
    fn mul(self, other: ProbabilityOutcome) -> ProbabilityOutcome {
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
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_mul_overflow() {
        let probability_outcome_one =
            ProbabilityOutcome::new_with_empty_constraint_map(ValueType::MAX);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(2);
        let _ = probability_outcome_one * probability_outcome_two;
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_mul_value_type_overflow() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(ValueType::MAX);
        let _ = probability_outcome * 2;
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_value_type_mul_overflow() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(2);
        let _ = ValueType::MAX * probability_outcome;
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_mul_underflow() {
        let probability_outcome_one =
            ProbabilityOutcome::new_with_empty_constraint_map(ValueType::MAX);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(-2);
        let _ = probability_outcome_one * probability_outcome_two;
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_mul_value_type_underflow() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(ValueType::MAX);
        let _ = probability_outcome * -2;
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_value_type_mul_underflow() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(-2);
        let _ = ValueType::MAX * probability_outcome;
    }
}
