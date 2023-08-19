use crate::constraint_management::{Constraint, ConstraintMap};
use crate::probability::ProbabilityOutcome;
use crate::ValueType;

#[allow(dead_code)]
impl ProbabilityOutcome {
    /// Creates a new [ProbabilityOutcome] with an empty [ConstraintMap].
    ///
    /// # Arguments
    ///
    /// * `value`: a [ValueType] to represent this outcome.
    ///
    /// # Returns
    ///
    /// The new empty [ProbabilityOutcome].
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityOutcome;
    /// let value = 123;
    /// let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(value);
    /// ```
    pub fn new_with_empty_constraint_map(value: ValueType) -> ProbabilityOutcome {
        ProbabilityOutcome {
            value,
            constraint_map: ConstraintMap::new_empty_constraint_map(),
        }
    }

    /// Creates a new [ProbabilityOutcome] with given [ConstraintMap].
    ///
    /// # Arguments
    ///
    /// * `value`: a [ValueType] to represent this outcome.
    /// * `constraint_map`: a [ConstraintMap] to represent the constraints of this outcome.
    ///
    /// # Returns
    ///
    /// The new empty [ProbabilityOutcome].
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityOutcome;
    /// # use crate::rusted_dice::constraint_management::ConstraintMap;
    /// let value = 123;
    /// let constraint_map = ConstraintMap::new_empty_constraint_map();
    /// let probability_outcome = ProbabilityOutcome::new_with_constraint_map(value, constraint_map);
    /// ```
    pub fn new_with_constraint_map(
        value: ValueType,
        constraint_map: ConstraintMap,
    ) -> ProbabilityOutcome {
        ProbabilityOutcome {
            value,
            constraint_map,
        }
    }

    /// Creates a new [ProbabilityOutcome] with given [ConstraintMap].
    /// Constraints with the same key will be combined see ConstraintMap::new_constraint_map
    ///
    /// # Arguments
    ///
    /// * `value`: a [ValueType] to represent this outcome.
    /// * `constraints`: an iterator of [Constraint]s to represent the constraints of this outcome.
    ///
    /// # Returns
    ///
    /// The new empty [ProbabilityOutcome].
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityOutcome;
    /// # use crate::rusted_dice::constraint_management::Constraint;
    /// let value = 123;
    /// let constraint_1 = Constraint::new_many_item_constraint(1, vec![1, 2, 3]);
    /// let constraint_2 = Constraint::new_many_item_constraint(2, vec![1, 2, 3]);
    /// let probability_outcome = ProbabilityOutcome::new_with_constraints(
    ///     value,
    ///     vec![constraint_1, constraint_2]
    /// );
    /// ```
    pub fn new_with_constraints(
        value: ValueType,
        constraints: impl IntoIterator<Item = Constraint>,
    ) -> ProbabilityOutcome {
        ProbabilityOutcome {
            value,
            constraint_map: ConstraintMap::new_constraint_map(constraints),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        constraint_management::{Constraint, ConstraintMap},
        probability::ProbabilityOutcome,
        ValueType,
    };
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_new_empty_constraint(test_value: ValueType) {
            let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(test_value);
            assert!(probability_outcome.value == test_value);
            assert_eq!(probability_outcome.constraint_map, ConstraintMap::new_empty_constraint_map());
        }

        #[test]
        fn test_new_with_constraint_map(test_value: ValueType) {
            let test_constraint_map = ConstraintMap::new_constraint_map(vec![
                Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
                Constraint::new_many_item_constraint(3, vec![1, 2, 3]),
            ]);
            let probability_outcome = ProbabilityOutcome::new_with_constraint_map(test_value, test_constraint_map.clone());
            assert!(probability_outcome.value == test_value);
            assert_eq!(probability_outcome.constraint_map, test_constraint_map);
        }

        #[test]
        fn test_new_with_constraints(test_value: ValueType) {
            let test_constraints = vec![
                Constraint::new_many_item_constraint(1, vec![1]),
                Constraint::new_many_item_constraint(2, vec![1, 2]),
                Constraint::new_many_item_constraint(3, vec![1, 2, 3]),
                Constraint::new_many_item_constraint(4, vec![1, 2, 3, 4]),
            ];
            let probability_outcome = ProbabilityOutcome::new_with_constraints(test_value, test_constraints.clone());
            assert!(probability_outcome.value == test_value);
            assert_eq!(probability_outcome.constraint_map, ConstraintMap::new_constraint_map(test_constraints));
        }

        #[test]
        fn test_new_with_constraints_empty(test_value: ValueType) {
            let probability_outcome = ProbabilityOutcome::new_with_constraints(test_value, vec![]);
            assert!(probability_outcome.value == test_value);
            assert_eq!(probability_outcome.constraint_map, ConstraintMap::new_constraint_map(vec![]));
        }
    }
}
