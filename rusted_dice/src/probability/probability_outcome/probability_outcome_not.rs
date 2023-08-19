use std::ops::Not;

use crate::probability::ProbabilityOutcome;

impl Not for ProbabilityOutcome {
    type Output = Self;

    /// Implements the bitwise not operator for [ProbabilityOutcome].
    /// values is negated.
    /// constraint map is unchanged.
    ///
    /// # Arguments
    ///
    /// * `self` - The [ProbabilityOutcome].
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityOutcome] after the bitwise not operation.
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
    ///     123, constraint_map.clone()
    /// );
    ///
    /// let probability_outcome_two = ProbabilityOutcome::new_with_constraint_map(
    ///     -124, constraint_map
    /// );
    ///
    /// assert_eq!(!probability_outcome_one, probability_outcome_two);
    /// ```
    fn not(self) -> Self {
        Self {
            value: !self.value,
            constraint_map: self.constraint_map,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::constraint_management::{Constraint, ConstraintMap};
    use crate::probability::ProbabilityOutcome;
    use crate::ValueType;

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_not(value_one: ValueType) {
            let expected_value = !value_one;
            let probability_outcome = ProbabilityOutcome::new_with_constraint_map(
                value_one,
                ConstraintMap::new_constraint_map(
                    vec![Constraint::new_many_item_constraint(3, vec![1, 2, 3])]
                )
            );
            let result = !probability_outcome;
            assert_eq!(result.value, expected_value);
            assert_eq!(
                result.constraint_map,
                ConstraintMap::new_constraint_map(
                    vec![
                        Constraint::new_many_item_constraint(
                            3,
                            vec![1, 2, 3]
                        )
                    ]
                )
            );
        }
    }
}
