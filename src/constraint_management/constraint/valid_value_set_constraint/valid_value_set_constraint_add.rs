use std::ops::Add;

use crate::ValueType;
use crate::constraint_management::{combine_valid_value_sets, ValidValueSetConstraint};

impl<T> Add for ValidValueSetConstraint<T>
where
    T: ValueType,
{
    type Output = Self;

    /// When adding two [ValidValueSetConstraint] the result will be the subset of both valid values
    ///
    /// # Panics
    ///
    /// Panics if the `id` of `self` does not match the `id` of `other`.
    ///
    /// # Arguments
    ///
    /// * `self` - The first [ValidValueSetConstraint] operand.
    /// * `other` - The second [ValidValueSetConstraint] operand.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::constraint_management::ValidValueSetConstraint;
    /// let constraint_set_one = ValidValueSetConstraint::new_many_item_constraint(2, vec![3, 4, 5]);
    /// let constraint_set_two = ValidValueSetConstraint::new_many_item_constraint(2, vec![1, 2, 3]);
    /// let constraint_set_three = ValidValueSetConstraint::new_single_valid_value_constraint(2, 3);
    ///
    /// assert_eq!(constraint_set_one + constraint_set_two, constraint_set_three);
    /// ```
    fn add(self, other: Self) -> Self {
        if self.id != other.id {
            panic!("Can not combine Constraints with different ids.");
        }
        ValidValueSetConstraint {
            id: self.id,
            valid_values: combine_valid_value_sets::<T>(&self.valid_values, &other.valid_values),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::constraint_management::ValidValueSetConstraint;

    #[test]
    #[should_panic(expected = "Can not combine Constraints with different ids.")]
    fn panic_on_different_id_combine() {
        let constraint = ValidValueSetConstraint::<i8>::new_empty_constraint(0);
        _ = constraint + ValidValueSetConstraint::new_empty_constraint(1);
    }

    #[test]
    fn combine_no_overlap() {
        let expected_value: HashSet<u16> = vec![].into_iter().collect();
        let mut constraint =
            ValidValueSetConstraint::new_many_item_constraint(1234, vec![1, 3, 5, 6]);
        constraint =
            constraint + ValidValueSetConstraint::new_single_valid_value_constraint(1234, 4);

        assert_eq!(
            constraint.valid_values.difference(&expected_value).count(),
            0
        );
        assert_eq!(constraint.id, 1234);
    }

    #[test]
    fn combine_part_overlap() {
        let expected_value: HashSet<char> = vec!['a', 'c'].into_iter().collect();
        let mut constraint =
            ValidValueSetConstraint::new_many_item_constraint(1234, vec!['a', 'b', 'c', 'd']);
        constraint = constraint
            + ValidValueSetConstraint::new_many_item_constraint(1234, vec!['a', 'c', 'x']);

        assert_eq!(
            constraint.valid_values.difference(&expected_value).count(),
            0
        );
        assert_eq!(constraint.id, 1234);
    }

    #[test]
    fn combine_full_overlap() {
        let expected_value: HashSet<u16> = vec![4, 5, 6].into_iter().collect();
        let mut constraint = ValidValueSetConstraint::new_many_item_constraint(1234, vec![4, 5, 6]);
        constraint =
            constraint + ValidValueSetConstraint::new_many_item_constraint(1234, vec![4, 5, 6]);

        assert_eq!(
            constraint.valid_values.difference(&expected_value).count(),
            0
        );
        assert_eq!(constraint.id, 1234);
    }
}
