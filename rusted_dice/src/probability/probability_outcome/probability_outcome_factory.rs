use crate::constraint_management::{Constraint, ConstraintMap, ConstraintValueType};
use crate::probability::ProbabilityOutcome;

#[allow(dead_code)]
impl ProbabilityOutcome {
    pub(crate) fn new_with_empty_constraint_map(value: ConstraintValueType) -> ProbabilityOutcome {
        ProbabilityOutcome {
            value,
            constraint_map: ConstraintMap::new_empty_constraint_map(),
        }
    }

    pub(crate) fn new_with_constraint_map(
        value: ConstraintValueType,
        constraint_map: ConstraintMap,
    ) -> ProbabilityOutcome {
        ProbabilityOutcome {
            value,
            constraint_map,
        }
    }

    pub(crate) fn new_with_constraints(
        value: ConstraintValueType,
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
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_new_empty_constraint(test_value: ConstraintValueType) {
            let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(test_value);
            assert!(probability_outcome.value == test_value);
            assert_eq!(probability_outcome.constraint_map, ConstraintMap::new_empty_constraint_map());
        }

        #[test]
        fn test_new_with_constraint_map(test_value: ConstraintValueType) {
            let test_constraint_map = ConstraintMap::new_constraint_map(vec![
                Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
                Constraint::new_many_item_constraint(3, vec![1, 2, 3]),
            ]);
            let probability_outcome = ProbabilityOutcome::new_with_constraint_map(test_value, test_constraint_map.clone());
            assert!(probability_outcome.value == test_value);
            assert_eq!(probability_outcome.constraint_map, test_constraint_map);
        }

        #[test]
        fn test_new_with_constraints(test_value: ConstraintValueType) {
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
        fn test_new_with_constraints_empty(test_value: ConstraintValueType) {
            let probability_outcome = ProbabilityOutcome::new_with_constraints(test_value, vec![]);
            assert!(probability_outcome.value == test_value);
            assert_eq!(probability_outcome.constraint_map, ConstraintMap::new_constraint_map(vec![]));
        }
    }
}
