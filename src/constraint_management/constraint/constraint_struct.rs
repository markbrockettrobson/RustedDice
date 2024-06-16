use crate::constraint_management::{ConstraintIdType, ValueTypeSet};

/// Represents a [Constraint] with an ID and a set of valid values.
///
/// Each [Constraint] has a [ConstraintIdType] (`id`) and a [ValueTypeSet] (`valid_values`),
/// which contains the allowed values for the [Constraint].
///
/// [Constraint]s are utilized to express values within a ProbabilityDistribution that cannot be combined due to their derivation from the same random event.
/// see ProbabilityOutcome for use case.
///
/// # Examples
/// #### A [Constraint] with no valid values. Mostly used for testing
/// ```
/// # use crate::rusted_dice::constraint_management::Constraint;
/// let constraint = Constraint::new_empty_constraint(1);
/// ```
///
/// #### A [Constraint] with one valid value
/// ```
/// # use crate::rusted_dice::constraint_management::Constraint;
/// let constraint = Constraint::new_single_valid_value_constraint(1, 20);
/// ```
///
/// #### A [Constraint] with many valid values
/// ```
/// # use crate::rusted_dice::constraint_management::Constraint;
/// let constraint = Constraint::new_many_item_constraint(1, vec![1, 2, 3]);
/// ```
///
/// #### Raw [Constraint]
/// ```
/// # use crate::rusted_dice::constraint_management::Constraint;
/// # use crate::rusted_dice::constraint_management::ValueTypeSet;
/// let values: ValueTypeSet = vec![1, 2, 3, 4].into_iter().collect();
/// let constraint = Constraint { id: 1, valid_values: values };
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Constraint {
    pub id: ConstraintIdType,
    pub valid_values: ValueTypeSet,
}

#[cfg(test)]
mod tests {
    use crate::ValueType;

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
        fn test_fmt(test_id: ConstraintIdType, test_valid_value: ValueType) {
            let constraint = Constraint::new_many_item_constraint(test_id, vec![test_valid_value]);
            assert_eq!(format!("{constraint:?}"), format!("Constraint {{ id: {}, valid_values: {{{}}} }}", test_id, test_valid_value));
        }
    }
}
