use core::fmt::Debug;
use std::{collections::HashSet, hash::Hash};

#[cfg(test)]
use proptest_derive::Arbitrary;

use crate::constraint_management::ConstraintIdType;

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ValidValueSetConstraint<T>
where
    T: Eq + Hash + Debug + Ord,
{
    pub(in crate::constraint_management::constraint::valid_value_set_constraint) id:
        ConstraintIdType,
    pub(in crate::constraint_management::constraint::valid_value_set_constraint) valid_values:
        HashSet<T>,
}

impl<T> ValidValueSetConstraint<T>
where
    T: Eq + Hash + Debug + Ord,
{
    pub fn get_id(&self) -> ConstraintIdType {
        self.id
    }

    pub fn get_valid_values(&self) -> &HashSet<T> {
        &self.valid_values
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    use crate::tests::test_hash_set_value_strategy;

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn test_clone_from() {
        let mut constraint_one =
            ValidValueSetConstraint::new_many_item_constraint(1234, vec![1, 3, 5]);
        let mut constraint_two =
            ValidValueSetConstraint::new_many_item_constraint(2, vec![2, 4, 6]);
        constraint_two.clone_from(&constraint_one);
        constraint_one.id = 20;
        assert_ne!(constraint_one.id, constraint_two.id);
        assert_ne!(constraint_two.id, 2);
        assert_eq!(constraint_one.valid_values, constraint_two.valid_values);
    }

    #[test]
    fn test_eq_true() {
        let constraint_one = ValidValueSetConstraint::new_many_item_constraint(2, vec![2, 4, 6]);
        let constraint_two = ValidValueSetConstraint::new_many_item_constraint(2, vec![2, 4, 6]);
        assert!(constraint_one == constraint_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_id() {
        let constraint_one =
            ValidValueSetConstraint::new_many_item_constraint(1234, vec!['2', '4', '6']);
        let constraint_two =
            ValidValueSetConstraint::new_many_item_constraint(2, vec!['2', '4', '6']);
        assert!(constraint_one != constraint_two);
    }

    proptest! {
        #[test]
        #[allow(clippy::nonminimal_bool)]
        fn test_eq_false_hashset(
            test_id: ConstraintIdType,
            test_valid_values in test_hash_set_value_strategy(2, 5)
        ) {
            let constraint_one =
                ValidValueSetConstraint::new_many_item_constraint(
                    test_id,
                    test_valid_values.iter().collect::<Vec<_>>()[1..].to_vec());
            let constraint_two =
                ValidValueSetConstraint::new_many_item_constraint(
                    test_id,
                    test_valid_values.iter().collect::<Vec<_>>());
            assert!(constraint_one != constraint_two);
        }

        #[test]
        #[allow(clippy::clone_on_copy)]
        fn test_clone(mut constraint_one: ValidValueSetConstraint<char>) {
            let constraint_two = constraint_one.clone();
            assert_eq!(constraint_one.id, constraint_two.id);
            assert_eq!(constraint_one.valid_values, constraint_two.valid_values);
        }
    }
}
