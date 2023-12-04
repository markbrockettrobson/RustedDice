use std::ops::Add;

use crate::{constraint_management::Constraint, probability::ProbabilityOutcome};

impl Add<Constraint> for ProbabilityOutcome {
    type Output = Self;

    /// Implements the addition operator for [ProbabilityOutcome] and [Constraint].
    /// The [Constraint] is added to the [ProbabilityOutcome]'s constraint map.
    ///
    /// # Arguments
    ///
    /// * `self` - The first [ProbabilityOutcome] operand.
    /// * `other` - The second [Constraint] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityOutcome] after the addition operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::constraint_management::Constraint;
    /// # use crate::rusted_dice::probability::ProbabilityOutcome;
    /// let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(123);
    /// let constraint = Constraint::new_many_item_constraint(1, vec![1, 2, 3]);
    ///
    /// let probability_outcome_with_constraint = probability_outcome + constraint.clone();
    ///
    /// assert_eq!(
    ///     probability_outcome_with_constraint,
    ///     ProbabilityOutcome::new_with_constraints(123, vec![constraint])
    /// );
    /// ```
    fn add(mut self, constraint: Constraint) -> Self {
        self.constraint_map += constraint;
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{constraint_management::Constraint, probability::ProbabilityOutcome};

    #[test]
    fn test_add_to_empty_constraint_map() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(123);
        let constraint = Constraint::new_many_item_constraint(1, vec![1, 2, 3]);

        let probability_outcome_with_constraint = probability_outcome + constraint.clone();

        assert_eq!(
            probability_outcome_with_constraint,
            ProbabilityOutcome::new_with_constraints(123, vec![constraint])
        );
    }

    #[test]
    fn test_add_to_non_empty_constraint_map_no_overlap() {
        let constraint_one = Constraint::new_many_item_constraint(1, vec![1, 2, 3]);
        let constraint_two = Constraint::new_many_item_constraint(2, vec![100, 200, 300]);

        let probability_outcome =
            ProbabilityOutcome::new_with_constraints(123, vec![constraint_one.clone()]);

        let probability_outcome_with_constraint = probability_outcome + constraint_two.clone();

        assert_eq!(
            probability_outcome_with_constraint,
            ProbabilityOutcome::new_with_constraints(123, vec![constraint_one, constraint_two])
        );
    }

    #[test]
    fn test_add_to_non_empty_constraint_with_overlap() {
        let constraint_one = Constraint::new_many_item_constraint(1, vec![1, 2, 3]);
        let constraint_two = Constraint::new_many_item_constraint(1, vec![3, 4, 5]);

        let probability_outcome =
            ProbabilityOutcome::new_with_constraints(123, vec![constraint_one]);

        let probability_outcome_with_constraint = probability_outcome + constraint_two;

        assert_eq!(
            probability_outcome_with_constraint,
            ProbabilityOutcome::new_with_constraints(
                123,
                vec![Constraint::new_single_valid_value_constraint(1, 3)]
            )
        );
    }
}
