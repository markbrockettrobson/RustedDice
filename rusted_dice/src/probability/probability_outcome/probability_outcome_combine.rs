use crate::probability::{BinaryOperation, Combine, ProbabilityOutcome};

impl Combine for ProbabilityOutcome {
    fn combine(&self, other: Self, binary_operation: BinaryOperation) -> Self {
        ProbabilityOutcome {
            value: binary_operation(self.value, other.value),
            constraint_map: self.constraint_map.clone() + other.constraint_map,
        }
    }
    fn combinei32(&self, other: i32, binary_operation: BinaryOperation) -> Self {
        ProbabilityOutcome {
            value: binary_operation(self.value, other),
            constraint_map: self.constraint_map.clone(),
        }
    }
    fn i32combine(&self, other: i32, binary_operation: BinaryOperation) -> Self {
        ProbabilityOutcome {
            value: binary_operation(other, self.value),
            constraint_map: self.constraint_map.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::constraint_management::{
        Constraint, ConstraintIdType, ConstraintMap, ConstraintValueType,
    };
    use crate::probability::{Combine, ProbabilityOutcome};

    fn has_key_valid_value(
        constraint_map: &ConstraintMap,
        id: ConstraintIdType,
        valid_value: ConstraintValueType,
    ) -> bool {
        constraint_map
            .map
            .get(&id)
            .unwrap()
            .valid_values
            .contains(&valid_value)
    }

    fn key_valid_value_len(constraint_map: &ConstraintMap, id: ConstraintIdType) -> usize {
        constraint_map.map.get(&id).unwrap().valid_values.len()
    }

    #[test]
    fn test_combine_constraint_map() {
        let probability_outcome_one = ProbabilityOutcome::new_with_constraints(
            123,
            vec![Constraint::new_many_item_constraint(1, vec![1, 2])],
        );
        let probability_outcome_two = ProbabilityOutcome::new_with_constraints(
            123,
            vec![Constraint::new_many_item_constraint(1, vec![2, 3])],
        );

        let combined_probability_outcome =
            probability_outcome_one.combine(probability_outcome_two, |lhs, rhs| lhs + rhs);

        let combined_constraint_map = combined_probability_outcome.constraint_map;

        assert_eq!(combined_probability_outcome.value, 246);
        assert_eq!(combined_constraint_map.map.len(), 1);
        assert_eq!(key_valid_value_len(&combined_constraint_map, 1), 1);
        assert!(!has_key_valid_value(&combined_constraint_map, 1, 1));
        assert!(has_key_valid_value(&combined_constraint_map, 1, 2));
        assert!(!has_key_valid_value(&combined_constraint_map, 1, 3));
    }

    #[test]
    fn test_combine_constrainti32_map() {
        let probability_outcome = ProbabilityOutcome::new_with_constraints(
            10,
            vec![Constraint::new_many_item_constraint(1, vec![1, 2])],
        );
        let combined_probability_outcome = probability_outcome.combinei32(1, |lhs, rhs| lhs - rhs);
        let combined_constraint_map = combined_probability_outcome.constraint_map;

        assert_eq!(combined_probability_outcome.value, 9);
        assert_eq!(combined_constraint_map.map.len(), 1);
        assert_eq!(key_valid_value_len(&combined_constraint_map, 1), 2);
        assert!(has_key_valid_value(&combined_constraint_map, 1, 1));
        assert!(has_key_valid_value(&combined_constraint_map, 1, 2));
    }

    #[test]
    fn test_combine_i32constraint_map() {
        let probability_outcome = ProbabilityOutcome::new_with_constraints(
            10,
            vec![Constraint::new_many_item_constraint(2, vec![1, 2])],
        );
        let combined_probability_outcome = probability_outcome.i32combine(1, |lhs, rhs| lhs - rhs);
        let combined_constraint_map = combined_probability_outcome.constraint_map;

        assert_eq!(combined_probability_outcome.value, -9);
        assert_eq!(combined_constraint_map.map.len(), 1);
        assert_eq!(key_valid_value_len(&combined_constraint_map, 2), 2);
        assert!(has_key_valid_value(&combined_constraint_map, 2, 1));
        assert!(has_key_valid_value(&combined_constraint_map, 2, 2));
    }
}
