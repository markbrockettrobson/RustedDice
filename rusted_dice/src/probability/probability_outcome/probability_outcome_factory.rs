use crate::constraint_management::{
    Constraint, ConstraintMap, ConstraintMapFactory, ConstraintValueType,
};
use crate::probability::ProbabilityOutcome;

#[allow(dead_code)]
pub struct ProbabilityOutcomeFactory;

#[allow(dead_code)]
impl ProbabilityOutcomeFactory {
    pub(crate) fn new_with_empty_constraint_map(value: ConstraintValueType) -> ProbabilityOutcome {
        ProbabilityOutcome {
            value,
            constraint_map: ConstraintMapFactory::new_empty_constraint_map(),
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
            constraint_map: ConstraintMapFactory::new_constraint_map(constraints),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constraint_management::ConstraintFactory;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_new_empty_constraint(test_value: ConstraintValueType) {
            let probability_outcome = ProbabilityOutcomeFactory::new_with_empty_constraint_map(test_value);
            assert!(probability_outcome.value == test_value);
            assert_eq!(probability_outcome.constraint_map, ConstraintMapFactory::new_empty_constraint_map());
        }

        #[test]
        fn test_new_with_constraint_map(test_value: ConstraintValueType) {
            let test_constraint_map = ConstraintMapFactory::new_constraint_map(vec![
                ConstraintFactory::new_many_item_constraint(2, vec![1, 2, 3]),
                ConstraintFactory::new_many_item_constraint(3, vec![1, 2, 3]),
            ]);
            let probability_outcome = ProbabilityOutcomeFactory::new_with_constraint_map(test_value, test_constraint_map.clone());
            assert!(probability_outcome.value == test_value);
            assert_eq!(probability_outcome.constraint_map, test_constraint_map);
        }

        #[test]
        fn test_new_with_constraints(test_value: ConstraintValueType) {
            let test_constraints = vec![
                ConstraintFactory::new_many_item_constraint(1, vec![1]),
                ConstraintFactory::new_many_item_constraint(2, vec![1, 2]),
                ConstraintFactory::new_many_item_constraint(3, vec![1, 2, 3]),
                ConstraintFactory::new_many_item_constraint(4, vec![1, 2, 3, 4]),
            ];
            let probability_outcome = ProbabilityOutcomeFactory::new_with_constraints(test_value, test_constraints.clone());
            assert!(probability_outcome.value == test_value);
            assert_eq!(probability_outcome.constraint_map, ConstraintMapFactory::new_constraint_map(test_constraints));
        }

        #[test]
        fn test_new_with_constraints_empty(test_value: ConstraintValueType) {
            let probability_outcome = ProbabilityOutcomeFactory::new_with_constraints(test_value, vec![]);
            assert!(probability_outcome.value == test_value);
            assert_eq!(probability_outcome.constraint_map, ConstraintMapFactory::new_constraint_map(vec![]));
        }
    }
}
