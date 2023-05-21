use std::ops::Add;

use super::Constraint;

impl Add for Constraint {
    type Output = Self;

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
    use super::super::ConstraintFactory;
    use super::super::ConstraintValueType;
    use std::collections::HashSet;

    #[test]
    #[should_panic(expected = "Can not combine Constraints with different ids.")]
    fn panic_on_different_id_combine() {
        let constraint_one = ConstraintFactory::new_empty_constraint(0);
        let constraint_two = ConstraintFactory::new_empty_constraint(1);
        let _ = constraint_one + constraint_two;
    }

    #[test]
    fn combine_no_overlap() {
        let expected_value: HashSet<ConstraintValueType> = vec![].into_iter().collect();
        let constraint_one = ConstraintFactory::new_many_item_constraint(1234, vec![1, 3, 5, 6]);
        let constraint_two = ConstraintFactory::new_single_valid_value_constraint(1234, 4);

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
        let expected_value: HashSet<ConstraintValueType> = vec![5, 6].into_iter().collect();
        let constraint_one = ConstraintFactory::new_many_item_constraint(1234, vec![1, 3, 5, 6]);
        let constraint_two = ConstraintFactory::new_many_item_constraint(1234, vec![4, 5, 6]);

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
        let expected_value: HashSet<ConstraintValueType> = vec![4, 5, 6].into_iter().collect();
        let constraint_one = ConstraintFactory::new_many_item_constraint(1234, vec![4, 5, 6]);
        let constraint_two = ConstraintFactory::new_many_item_constraint(1234, vec![4, 5, 6]);

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
