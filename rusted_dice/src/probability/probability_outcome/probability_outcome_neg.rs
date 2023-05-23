use std::ops::Neg;

use crate::probability::ProbabilityOutcome;

impl Neg for ProbabilityOutcome {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            value: -self.value,
            constraint_map: self.constraint_map,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::constraint_management::{Constraint, ConstraintMap};
    use crate::probability::ProbabilityOutcome;

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_neg(value_one: i32) {
            let expected_value = -value_one;
            let probability_outcome = ProbabilityOutcome::new_with_constraint_map(
                value_one,
                ConstraintMap::new_constraint_map(
                    vec![Constraint::new_many_item_constraint(3, vec![1, 2, 3])]
                )
            );
            let result = -probability_outcome;
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
