use crate::{
    probability::{BinaryOperation, Combine, ProbabilityOutcome},
    ValueType,
};

impl Combine for ProbabilityOutcome {
    
    /// Combine this instance with another instance using the specified [BinaryOperation].
    /// in the order: self [BinaryOperation] `other`
    ///
    /// values are combined using the [BinaryOperation] function.
    /// constraint maps are combined using the ConstraintMap::add function.
    ///
    /// # Arguments
    ///
    /// * `other` - The self type to check preform the [BinaryOperation] with.
    /// * `binary_operation` - the [BinaryOperation] function.
    ///
    /// # Returns
    ///
    /// Returns the [ProbabilityOutcome] type result of the [BinaryOperation] function.
    fn combine(&self, other: Self, binary_operation: BinaryOperation) -> Self {
        ProbabilityOutcome {
            value: binary_operation(self.value, other.value),
            constraint_map: self.constraint_map.clone() + other.constraint_map,
        }
    }

    /// Combine this instance with a [ValueType] using the specified [BinaryOperation].
    /// in the order: self [BinaryOperation] [ValueType]
    ///
    /// values are combined using the [BinaryOperation] function.
    /// constraint maps are combined using the ConstraintMap::add function.
    ///
    /// # Arguments
    ///
    /// * `other` - The [ValueType] to check preform the [BinaryOperation] with.
    /// * `binary_operation` - the [BinaryOperation] function.
    ///
    /// # Returns
    ///
    /// Returns the [ProbabilityOutcome] type result of the [BinaryOperation] function.
    fn combine_value_type(&self, other: ValueType, binary_operation: BinaryOperation) -> Self {
        ProbabilityOutcome {
            value: binary_operation(self.value, other),
            constraint_map: self.constraint_map.clone(),
        }
    }

    /// Combine this instance with a [ValueType] using the specified [BinaryOperation].
    /// in the order: [ValueType] [BinaryOperation] self
    ///
    /// values are combined using the [BinaryOperation] function.
    /// constraint maps are combined using the ConstraintMap::add function.
    ///
    /// # Arguments
    ///
    /// * `other` - The [ValueType] to check preform the [BinaryOperation] with.
    /// * `binary_operation` - the [BinaryOperation] function.
    ///
    /// # Returns
    ///
    /// Returns the [ProbabilityOutcome] type result of the [BinaryOperation] function.
    fn value_type_combine(&self, other: ValueType, binary_operation: BinaryOperation) -> Self {
        ProbabilityOutcome {
            value: binary_operation(other, self.value),
            constraint_map: self.constraint_map.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::constraint_management::{Constraint, ConstraintIdType, ConstraintMap};
    use crate::probability::{Combine, ProbabilityOutcome};
    use crate::ValueType;

    fn has_key_valid_value(
        constraint_map: &ConstraintMap,
        id: ConstraintIdType,
        valid_value: ValueType,
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
    fn test_combine_constraint_value_type_map() {
        let probability_outcome = ProbabilityOutcome::new_with_constraints(
            10,
            vec![Constraint::new_many_item_constraint(1, vec![1, 2])],
        );
        let combined_probability_outcome =
            probability_outcome.combine_value_type(1, |lhs, rhs| lhs - rhs);
        let combined_constraint_map = combined_probability_outcome.constraint_map;

        assert_eq!(combined_probability_outcome.value, 9);
        assert_eq!(combined_constraint_map.map.len(), 1);
        assert_eq!(key_valid_value_len(&combined_constraint_map, 1), 2);
        assert!(has_key_valid_value(&combined_constraint_map, 1, 1));
        assert!(has_key_valid_value(&combined_constraint_map, 1, 2));
    }

    #[test]
    fn test_combine_value_type_constraint_map() {
        let probability_outcome = ProbabilityOutcome::new_with_constraints(
            10,
            vec![Constraint::new_many_item_constraint(2, vec![1, 2])],
        );
        let combined_probability_outcome =
            probability_outcome.value_type_combine(1, |lhs, rhs| lhs - rhs);
        let combined_constraint_map = combined_probability_outcome.constraint_map;

        assert_eq!(combined_probability_outcome.value, -9);
        assert_eq!(combined_constraint_map.map.len(), 1);
        assert_eq!(key_valid_value_len(&combined_constraint_map, 2), 2);
        assert!(has_key_valid_value(&combined_constraint_map, 2, 1));
        assert!(has_key_valid_value(&combined_constraint_map, 2, 2));
    }
}
