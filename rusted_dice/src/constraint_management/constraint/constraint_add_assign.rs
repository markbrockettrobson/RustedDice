use std::ops::AddAssign;

use crate::constraint_management::combine_valid_value_sets;
use crate::constraint_management::Constraint;

impl AddAssign for Constraint {
    /// Implements the addition assignment operator for [Constraint]. The intersectoin of valid values is maintained
    ///
    /// # Panics
    ///
    /// Panics if the `id` of `self` does not match the `id` of `other`.
    ///
    /// # Arguments
    ///
    /// * `self` - The first [Constraint] operand.
    /// * `other` - The second [Constraint] operand.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::constraint_management::Constraint;
    /// let mut constraint_one = Constraint::new_many_item_constraint(2, vec![3, 4, 5]);
    /// constraint_one += Constraint::new_many_item_constraint(2, vec![1, 2, 3]);
    /// let constraint_two = Constraint::new_single_valid_value_constraint(2, 3);
    ///
    /// assert_eq!(constraint_one, constraint_two);
    /// ```
    fn add_assign(&mut self, other: Self) {
        if self.id != other.id {
            panic!("Can not combine Constraints with different ids.");
        }

        self.valid_values = combine_valid_value_sets(&self.valid_values, &other.valid_values);
    }
}

#[cfg(test)]
mod tests {
    use crate::constraint_management::Constraint;
    use crate::constraint_management::ValueTypeSet;

    #[test]
    #[should_panic(expected = "Can not combine Constraints with different ids.")]
    fn panic_on_different_id_combine() {
        let mut constraint = Constraint::new_empty_constraint(0);
        constraint += Constraint::new_empty_constraint(1);
    }

    #[test]
    fn combine_no_overlap() {
        let expected_value: ValueTypeSet = vec![].into_iter().collect();
        let mut constraint = Constraint::new_many_item_constraint(1234, vec![1, 3, 5, 6]);
        constraint += Constraint::new_single_valid_value_constraint(1234, 4);

        assert_eq!(
            constraint.valid_values.difference(&expected_value).count(),
            0
        );
        assert_eq!(constraint.id, 1234);
    }

    #[test]
    fn combine_part_overlap() {
        let expected_value: ValueTypeSet = vec![5, 6].into_iter().collect();
        let mut constraint = Constraint::new_many_item_constraint(1234, vec![1, 3, 5, 6]);
        constraint += Constraint::new_many_item_constraint(1234, vec![4, 5, 6]);

        assert_eq!(
            constraint.valid_values.difference(&expected_value).count(),
            0
        );
        assert_eq!(constraint.id, 1234);
    }

    #[test]
    fn combine_full_overlap() {
        let expected_value: ValueTypeSet = vec![4, 5, 6].into_iter().collect();
        let mut constraint = Constraint::new_many_item_constraint(1234, vec![4, 5, 6]);
        constraint += Constraint::new_many_item_constraint(1234, vec![4, 5, 6]);

        assert_eq!(
            constraint.valid_values.difference(&expected_value).count(),
            0
        );
        assert_eq!(constraint.id, 1234);
    }
}
