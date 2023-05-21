use crate::constraint_management::{ConstraintMapFactory, ConstraintValueType};
use crate::probability::ProbabilityOutcome;

#[allow(dead_code)]
pub struct ProbabilityOutcomeFactory;

#[allow(dead_code)]
impl ProbabilityOutcomeFactory {
    pub(crate) fn new_with_empty_constraint_map(value: ConstraintValueType) -> ProbabilityOutcome {
        let empty_constraint_map = ConstraintMapFactory::new_empty_constraint_map();
        ProbabilityOutcome {
            value,
            constraint_map: empty_constraint_map,
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
            let probability_outcome = ProbabilityOutcomeFactory::new_with_empty_constraint_map(test_value);
            assert!(probability_outcome.value == test_value);
            assert_eq!(probability_outcome.constraint_map.map.len(), 0);
        }
    }
}
