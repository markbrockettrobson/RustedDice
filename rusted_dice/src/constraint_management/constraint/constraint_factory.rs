use crate::constraint_management::{Constraint, ConstraintIDType, ConstraintValueType};
use std::collections::HashSet;

pub struct ConstraintFactory;

#[allow(dead_code)]
impl ConstraintFactory {
    pub(crate) fn new_empty_constraint(id: ConstraintIDType) -> Constraint {
        let valid_values: HashSet<ConstraintValueType> = HashSet::new();
        Constraint { id, valid_values }
    }

    pub(crate) fn new_single_valid_value_constraint(
        id: ConstraintIDType,
        value: ConstraintValueType,
    ) -> Constraint {
        let valid_values: HashSet<ConstraintValueType> = vec![value].into_iter().collect();
        Constraint { id, valid_values }
    }

    pub(crate) fn new_many_item_constraint(
        id: ConstraintIDType,
        values: impl IntoIterator<Item = ConstraintValueType>,
    ) -> Constraint {
        let valid_values: HashSet<ConstraintValueType> = values.into_iter().collect();
        Constraint { id, valid_values }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_new_empty_constraint(test_value: ConstraintIDType) {
            let test_valid_values: HashSet<ConstraintValueType> = HashSet::new();
            let constraint = Constraint {id: test_value, valid_values: test_valid_values };

            assert_eq!(constraint, ConstraintFactory::new_empty_constraint(test_value));
        }

        #[test]
        fn test_new_single_valid_value_constraint(test_value: ConstraintIDType, test_valid_value: ConstraintValueType) {
            let test_valid_values: HashSet<ConstraintValueType> = vec![test_valid_value].into_iter().collect();
            let constraint = Constraint {id: test_value, valid_values: test_valid_values };

            assert_eq!(constraint, ConstraintFactory::new_single_valid_value_constraint(test_value, test_valid_value));
        }

        #[test]
        fn test_new_many_item_constraint_iter(test_value: ConstraintIDType, test_valid_values: HashSet<ConstraintValueType>) {
            let constraint = Constraint {id: test_value, valid_values: test_valid_values.clone() };

            assert_eq!(constraint, ConstraintFactory::new_many_item_constraint(test_value, test_valid_values.into_iter()));
        }

        #[test]
        fn test_new_many_item_constraint_hashset(test_value: ConstraintIDType, test_valid_values: HashSet<ConstraintValueType>) {
            let constraint = Constraint {id: test_value, valid_values: test_valid_values.clone() };

            assert_eq!(constraint, ConstraintFactory::new_many_item_constraint(test_value, test_valid_values));
        }

        #[test]
        fn test_new_many_item_constraint_vec(test_value: u16) {
            let test_valid_values: HashSet<ConstraintValueType> = vec![2, 4, 6].into_iter().collect();
            let constraint = Constraint {id: test_value, valid_values: test_valid_values };

            assert_eq!(constraint, ConstraintFactory::new_many_item_constraint(test_value, vec![2, 4, 6]));
        }
    }
}
