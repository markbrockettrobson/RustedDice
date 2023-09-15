use crate::{
    probability::{Combine, ProbabilityOutcome},
    ValueType,
};
use std::ops::BitOr;

fn _bitor(lhs: ValueType, rhs: ValueType) -> ValueType {
    lhs | rhs
}

impl BitOr for ProbabilityOutcome {
    type Output = Self;

    /// Implements the bitwise or operator for [ProbabilityOutcome].
    /// values are combined using the bitwise or function.
    /// constraint maps are combined using the ConstraintMap::add function.
    ///
    /// # Arguments
    ///
    /// * `self` - The first [ProbabilityOutcome] operand.
    /// * `other` - The second [ProbabilityOutcome] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityOutcome] after the bitwise or operation.
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
    ///     236, constraint_map_three
    /// );
    ///
    /// assert_eq!(probability_outcome_one | probability_outcome_two, probability_outcome_three);
    /// ```
    fn bitor(self, other: Self) -> Self {
        self.combine(other, _bitor)
    }
}

impl BitOr<ValueType> for ProbabilityOutcome {
    type Output = Self;

    /// Implements the bitwise or operator for [ProbabilityOutcome] | [ValueType].
    /// values are combined using the bitwise or function.
    /// constraint map is taken from the [ProbabilityOutcome].
    ///
    /// # Arguments
    ///
    /// * `self` - The [ProbabilityOutcome] operand.
    /// * `other` - The [ValueType] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityOutcome] after the bitwise or operation.
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
    ///     236, constraint_map
    /// );
    ///
    /// assert_eq!(probability_outcome_one | 200, probability_outcome_two);
    /// ```
    fn bitor(self, other: ValueType) -> Self {
        self.combine_value_type(other, _bitor)
    }
}

impl BitOr<ProbabilityOutcome> for ValueType {
    type Output = ProbabilityOutcome;

    /// Implements the bitwise or operator for [ValueType] | [ProbabilityOutcome].
    /// values are combined using the bitwise or function.
    /// constraint map is taken from the [ProbabilityOutcome].
    ///
    /// # Arguments
    ///
    /// * `self` - The [ValueType] operand.
    /// * `other` - The [ProbabilityOutcome] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityOutcome] after the bitwise or operation.
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
    ///     200, constraint_map.clone()
    /// );
    ///
    /// let probability_outcome_two = ProbabilityOutcome::new_with_constraint_map(
    ///     236, constraint_map
    /// );
    ///
    /// assert_eq!(100 | probability_outcome_one, probability_outcome_two);
    /// ```
    fn bitor(self, other: ProbabilityOutcome) -> ProbabilityOutcome {
        other.value_type_combine(self, _bitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::{probability::ProbabilityOutcome, SmallValueType, ValueType};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_bitor(value_one: ValueType, value_two: ValueType) {
            let expected_value = value_one | value_two;
            let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(value_one);
            let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(value_two);
            let result = probability_outcome_one | probability_outcome_two;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_bitor_value_type(value_one: SmallValueType, value_two: SmallValueType) {
            let expected_value = ValueType::from(value_one) | ValueType::from(value_two);
            let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(value_one.into());
            let result = probability_outcome | ValueType::from(value_two);
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_value_type_bitor(value_one: SmallValueType, value_two: SmallValueType) {
            let expected_value = ValueType::from(value_one) | ValueType::from(value_two);
            let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(value_two.into());
            let result = ValueType::from(value_one) | probability_outcome;
            assert_eq!(result.value, expected_value);
        }
    }
}
