use std::ops::Div;

use crate::{
    probability::{Combine, ProbabilityOutcome},
    ValueType,
};

fn _div(lhs: ValueType, rhs: ValueType) -> ValueType {
    lhs / rhs
}

impl Div for ProbabilityOutcome {
    type Output = Self;

    /// Implements the division operator for [ProbabilityOutcome].
    /// values are combined using the division function.
    /// constraint maps are combined using the ConstraintMap::add function.
    ///
    /// # Arguments
    ///
    /// * `self` - The first [ProbabilityOutcome] operand.
    /// * `other` - The second [ProbabilityOutcome] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityOutcome] after the division operation.
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
    ///     20, constraint_map_two
    /// );
    ///
    /// let constraint_map_three = ConstraintMap::new_constraint_map(
    ///     vec![
    ///        Constraint::new_many_item_constraint(1, vec![3]),
    ///        Constraint::new_many_item_constraint(2, vec![1, 2, 3])
    ///     ]
    /// );
    /// let probability_outcome_three = ProbabilityOutcome::new_with_constraint_map(
    ///     5, constraint_map_three
    /// );
    ///
    /// assert_eq!(probability_outcome_one / probability_outcome_two, probability_outcome_three);
    /// ```
    fn div(self, other: Self) -> Self {
        self.combine(other, _div)
    }
}

impl Div<ValueType> for ProbabilityOutcome {
    type Output = Self;

    /// Implements the division operator for [ProbabilityOutcome] / [ValueType].
    /// values are combined using the division function.
    /// constraint map is taken from the [ProbabilityOutcome].
    ///
    /// # Arguments
    ///
    /// * `self` - The [ProbabilityOutcome] operand.
    /// * `other` - The [ValueType] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityOutcome] after the division operation.
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
    ///     5, constraint_map
    /// );
    ///
    /// assert_eq!(probability_outcome_one / 20, probability_outcome_two);
    /// ```
    fn div(self, other: ValueType) -> Self {
        self.combine_value_type(other, _div)
    }
}

impl Div<ProbabilityOutcome> for ValueType {
    type Output = ProbabilityOutcome;

    /// Implements the division operator for [ValueType] / [ProbabilityOutcome].
    /// values are combined using the division function.
    /// constraint map is taken from the [ProbabilityOutcome].
    ///
    /// # Arguments
    ///
    /// * `self` - The [ValueType] operand.
    /// * `other` - The [ProbabilityOutcome] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityOutcome] after the division operation.
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
    ///     300, constraint_map
    /// );
    ///
    /// assert_eq!(30000 / probability_outcome_one, probability_outcome_two);
    /// ```
    fn div(self, other: ProbabilityOutcome) -> ProbabilityOutcome {
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
