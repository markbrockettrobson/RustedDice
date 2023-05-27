use std::ops::Add;

use crate::constraint_management::Constraint;

impl Add for Constraint {
    type Output = Self;
    /// Implements the addition operator for `Constraint`. The intersectoin of valid values is maintained
    ///
    /// # Panics
    ///
    /// Panics if the `id` of `self` does not match the `id` of `other`.
    ///
    /// # Arguments
    ///
    /// * `self` - The first `Constraint` operand.
    /// * `other` - The second `Constraint` operand.
    ///
    /// # Returns
    ///
    /// The resulting `Constraint` after the addition operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::constraint_management::constraint::Constraint;
    /// let constraint_one = Constraint::new_many_item_constraint(2, vec![3, 4, 5]);
    /// let constraint_two = Constraint::new_many_item_constraint(2, vec![1, 2, 3]);
    /// let constraint_three = Constraint::new_single_valid_value_constraint(2, 3);
    ///
    /// assert_eq!(constraint_one + constraint_two, constraint_three);
    /// ```
    fn add(self, other: Self) -> Self {
        if self.id != other.id {
            panic!("Can not combine Constraints with different ids.");
        }
        Constraint {
            id: self.id,
            valid_values: self
                .valid_values
                .intersection(&other.valid_values)
                .copied()
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::constraint_management::Constraint;
    use crate::constraint_management::ValueTypeSet;

    #[test]
    #[should_panic(expected = "Can not combine Constraints with different ids.")]
    fn panic_on_different_id_combine() {
        let constraint_one = Constraint::new_empty_constraint(0);
        let constraint_two = Constraint::new_empty_constraint(1);
        let _ = constraint_one + constraint_two;
    }

    #[test]
    fn combine_no_overlap() {
        let expected_value: ValueTypeSet = vec![].into_iter().collect();
        let constraint_one = Constraint::new_many_item_constraint(1234, vec![1, 3, 5, 6]);
        let constraint_two = Constraint::new_single_valid_value_constraint(1234, 4);

        let constraint_three = constraint_one + constraint_two;

        assert_eq!(
            constraint_three
                .valid_values
                .difference(&expected_value)
                .count(),
            0
        );
        assert_eq!(constraint_three.id, 1234);
    }

    #[test]
    fn combine_part_overlap() {
        let expected_value: ValueTypeSet = vec![5, 6].into_iter().collect();
        let constraint_one = Constraint::new_many_item_constraint(1234, vec![1, 3, 5, 6]);
        let constraint_two = Constraint::new_many_item_constraint(1234, vec![4, 5, 6]);

        let constraint_three = constraint_one + constraint_two;

        assert_eq!(
            constraint_three
                .valid_values
                .difference(&expected_value)
                .count(),
            0
        );
        assert_eq!(constraint_three.id, 1234);
    }

    #[test]
    fn combine_full_overlap() {
        let expected_value: ValueTypeSet = vec![4, 5, 6].into_iter().collect();
        let constraint_one = Constraint::new_many_item_constraint(1234, vec![4, 5, 6]);
        let constraint_two = Constraint::new_many_item_constraint(1234, vec![4, 5, 6]);

        let constraint_three = constraint_one + constraint_two;

        assert_eq!(
            constraint_three
                .valid_values
                .difference(&expected_value)
                .count(),
            0
        );
        assert_eq!(constraint_three.id, 1234);
    }
}
