use std::ops::Not;

use crate::probability::ProbabilityOutcome;

impl Not for ProbabilityOutcome {
    type Output = Self;

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
