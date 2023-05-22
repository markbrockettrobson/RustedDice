use std::collections::HashSet;

use crate::constraint_management::{ConstraintIdType, ConstraintValueType};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Constraint {
    pub id: ConstraintIdType,
    pub valid_values: HashSet<ConstraintValueType>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn test_clone() {
        let mut constraint_one = Constraint::new_many_item_constraint(1234, vec![1, 3, 5]);
        let constraint_two = constraint_one.clone();
        constraint_one.id = 20;
        assert_ne!(constraint_one.id, constraint_two.id);
        assert_eq!(constraint_one.valid_values, constraint_two.valid_values);
    }

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn test_clone_from() {
        let mut constraint_one = Constraint::new_many_item_constraint(1234, vec![1, 3, 5]);
        let mut constraint_two = Constraint::new_many_item_constraint(2, vec![2, 4, 6]);
        constraint_two.clone_from(&constraint_one);
        constraint_one.id = 20;
        assert_ne!(constraint_one.id, constraint_two.id);
        assert_ne!(constraint_two.id, 2);
        assert_eq!(constraint_one.valid_values, constraint_two.valid_values);
    }

    #[test]
    fn test_eq_true() {
        let constraint_one = Constraint::new_many_item_constraint(2, vec![2, 4, 6]);
        let constraint_two = Constraint::new_many_item_constraint(2, vec![2, 4, 6]);
        assert!(constraint_one == constraint_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_id() {
        let constraint_one = Constraint::new_many_item_constraint(1234, vec![2, 4, 6]);
        let constraint_two = Constraint::new_many_item_constraint(2, vec![2, 4, 6]);
        assert!(constraint_one != constraint_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_hashset() {
        let constraint_one = Constraint::new_many_item_constraint(1234, vec![2, 4, 6]);
        let constraint_two = Constraint::new_many_item_constraint(1234, vec![2, 5, 6]);
        assert!(constraint_one != constraint_two);
    }

    proptest! {
        #[test]
        fn test_fmt(test_id: ConstraintIdType, test_valid_value: ConstraintValueType) {
            let constraint = Constraint::new_many_item_constraint(test_id, vec![test_valid_value]);
            assert_eq!(format!("{constraint:?}"), format!("Constraint {{ id: {}, valid_values: {{{}}} }}", test_id, test_valid_value));
        }
    }
}
