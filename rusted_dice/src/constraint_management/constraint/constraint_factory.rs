use crate::{
    constraint_management::{Constraint, ConstraintIdType, ValueTypeSet},
    ValueType,
};
use std::collections::HashSet;

#[allow(dead_code)]
impl Constraint {
    /// Creates a new empty constraint with the given ID.
    ///
    /// An empty constraint has no valid values associated with it.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier for the constraint.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::constraint_management::constraint::Constraint;
    /// let constraint = Constraint::new_empty_constraint(1);
    /// ```
    pub fn new_empty_constraint(id: ConstraintIdType) -> Constraint {
        let valid_values: ValueTypeSet = HashSet::new();
        Constraint { id, valid_values }
    }

    /// Creates a new constraint with a single valid value.
    ///
    /// The constraint allows only a single value to be valid.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier for the constraint.
    /// * `value` - The valid value for the constraint.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::constraint_management::constraint::Constraint;
    /// let constraint = Constraint::new_single_valid_value_constraint(2, 3);
    /// ```
    pub fn new_single_valid_value_constraint(id: ConstraintIdType, value: ValueType) -> Constraint {
        let valid_values: ValueTypeSet = vec![value].into_iter().collect();
        Constraint { id, valid_values }
    }

    /// Creates a new constraint with multiple valid values.
    ///
    /// The constraint allows multiple values to be valid.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier for the constraint.
    /// * `values` - An iterator over the valid values for the constraint.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::constraint_management::constraint::Constraint;
    /// let constraint = Constraint::new_many_item_constraint(3, vec![1, 2]);
    /// ```
    pub fn new_many_item_constraint(
        id: ConstraintIdType,
        values: impl IntoIterator<Item = ValueType>,
    ) -> Constraint {
        let valid_values: ValueTypeSet = values.into_iter().collect();
        Constraint { id, valid_values }
    }
}

#[cfg(test)]
mod tests {
    use crate::UnsignedSmallValueType;

    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_new_empty_constraint(test_value: ConstraintIdType) {
            let test_valid_values: ValueTypeSet = HashSet::new();
            let constraint = Constraint {id: test_value, valid_values: test_valid_values };

            assert_eq!(constraint, Constraint::new_empty_constraint(test_value));
        }

        #[test]
        fn test_new_single_valid_value_constraint(test_value: ConstraintIdType, test_valid_value: ValueType) {
            let test_valid_values: ValueTypeSet = vec![test_valid_value].into_iter().collect();
            let constraint = Constraint {id: test_value, valid_values: test_valid_values };

            assert_eq!(constraint, Constraint::new_single_valid_value_constraint(test_value, test_valid_value));
        }

        #[test]
        fn test_new_many_item_constraint_iter(test_value: ConstraintIdType, test_valid_values: ValueTypeSet) {
            let constraint = Constraint {id: test_value, valid_values: test_valid_values.clone() };

            assert_eq!(constraint, Constraint::new_many_item_constraint(test_value, test_valid_values.into_iter()));
        }

        #[test]
        fn test_new_many_item_constraint_hashset(test_value: ConstraintIdType, test_valid_values: ValueTypeSet) {
            let constraint = Constraint {id: test_value, valid_values: test_valid_values.clone() };

            assert_eq!(constraint, Constraint::new_many_item_constraint(test_value, test_valid_values));
        }

        #[test]
        fn test_new_many_item_constraint_vec(test_value: UnsignedSmallValueType) {
            let test_valid_values: ValueTypeSet = vec![2, 4, 6].into_iter().collect();
            let constraint = Constraint {id: test_value, valid_values: test_valid_values };

            assert_eq!(constraint, Constraint::new_many_item_constraint(test_value, vec![2, 4, 6]));
        }
    }
}
