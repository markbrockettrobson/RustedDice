use std::collections::BTreeMap;

use crate::constraint_management::{Constraint, ConstraintIdType};
use crate::probability::ProbabilityDistribution;

#[allow(dead_code)]
impl ProbabilityDistribution {
    /// Adds a constraint to the probability distribution that the value of the outcome must be equal to the constraint value.
    ///
    /// # Arguments
    ///
    /// * `constraint_id` - The id of the constraint to add.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::collections::BTreeMap;
    /// # use crate::rusted_dice::probability::ProbabilityOutcome;
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::constraint_management::{Constraint, ConstraintIdType};
    /// let constraint = Constraint::new_many_item_constraint(2, vec![10, 20, 30, 40, 50, 60]);
    ///     
    /// let mut b_tree_map = BTreeMap::new();
    /// b_tree_map.insert(
    ///     ProbabilityOutcome::new_with_constraints(30, vec![constraint]),
    ///     99,
    /// );
    /// let probability_distribution = ProbabilityDistribution {
    ///     outcome_counts: b_tree_map,
    /// };
    ///
    /// let probability_distribution_with_constraint = probability_distribution.add_self_value_constraint(2);
    ///
    /// assert_eq!(
    ///     probability_distribution_with_constraint.outcome_counts.get(
    ///         &ProbabilityOutcome::new_with_constraints(30, vec![Constraint::new_single_valid_value_constraint(2, 30)])
    ///     ),
    ///     Some(&99)
    /// );
    /// ```
    pub fn add_self_value_constraint(self, constraint_id: ConstraintIdType) -> Self {
        let mut b_tree_map = BTreeMap::new();

        self.outcome_counts
            .iter()
            .map(|(outcome, count)| {
                (
                    outcome.clone()
                        + Constraint::new_single_valid_value_constraint(
                            constraint_id,
                            outcome.value,
                        ),
                    count,
                )
            })
            .for_each(|(outcome, count)| {
                b_tree_map.insert(outcome, *count);
            });

        ProbabilityDistribution {
            outcome_counts: b_tree_map,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::probability::ProbabilityOutcome;

    use super::*;

    #[test]
    fn test_empty() {
        let probability_distribution = ProbabilityDistribution::new_empty_distribution();
        let probability_distribution_with_constraint =
            probability_distribution.add_self_value_constraint(20);

        assert_eq!(
            probability_distribution_with_constraint
                .outcome_counts
                .len(),
            0
        );
    }

    #[test]
    fn test_single_outcome_without_constraint() {
        let mut b_tree_map = BTreeMap::new();
        b_tree_map.insert(ProbabilityOutcome::new_with_empty_constraint_map(1111), 99);
        let probability_distribution = ProbabilityDistribution {
            outcome_counts: b_tree_map,
        };

        let probability_distribution_with_constraint =
            probability_distribution.add_self_value_constraint(20);

        assert_eq!(
            probability_distribution_with_constraint.outcome_counts.get(
                &ProbabilityOutcome::new_with_constraints(
                    1111,
                    vec![Constraint::new_single_valid_value_constraint(20, 1111)]
                )
            ),
            Some(&99)
        );

        assert_eq!(
            probability_distribution_with_constraint
                .outcome_counts
                .len(),
            1
        );
    }

    #[test]
    fn test_single_outcome_with_constraint() {
        let constraint = Constraint::new_many_item_constraint(2, vec![10, 20, 30, 40, 50, 60]);

        let mut b_tree_map = BTreeMap::new();
        b_tree_map.insert(
            ProbabilityOutcome::new_with_constraints(30, vec![constraint]),
            99,
        );
        let probability_distribution = ProbabilityDistribution {
            outcome_counts: b_tree_map,
        };

        let probability_distribution_with_constraint =
            probability_distribution.add_self_value_constraint(2);

        assert_eq!(
            probability_distribution_with_constraint.outcome_counts.get(
                &ProbabilityOutcome::new_with_constraints(
                    30,
                    vec![Constraint::new_single_valid_value_constraint(2, 30)]
                )
            ),
            Some(&99)
        );

        assert_eq!(
            probability_distribution_with_constraint
                .outcome_counts
                .len(),
            1
        );
    }

    #[test]
    fn test_many_different_outcomes() {
        let probability_distribution = ProbabilityDistribution::new_dice(4);

        let probability_distribution_with_constraint =
            probability_distribution.add_self_value_constraint(100);

        assert_eq!(
            probability_distribution_with_constraint.outcome_counts.get(
                &ProbabilityOutcome::new_with_constraints(
                    1,
                    vec![Constraint::new_single_valid_value_constraint(100, 1)]
                )
            ),
            Some(&1)
        );
        assert_eq!(
            probability_distribution_with_constraint.outcome_counts.get(
                &ProbabilityOutcome::new_with_constraints(
                    2,
                    vec![Constraint::new_single_valid_value_constraint(100, 2)]
                )
            ),
            Some(&1)
        );
        assert_eq!(
            probability_distribution_with_constraint.outcome_counts.get(
                &ProbabilityOutcome::new_with_constraints(
                    3,
                    vec![Constraint::new_single_valid_value_constraint(100, 3)]
                )
            ),
            Some(&1)
        );
        assert_eq!(
            probability_distribution_with_constraint.outcome_counts.get(
                &ProbabilityOutcome::new_with_constraints(
                    4,
                    vec![Constraint::new_single_valid_value_constraint(100, 4)]
                )
            ),
            Some(&1)
        );

        assert_eq!(
            probability_distribution_with_constraint
                .outcome_counts
                .len(),
            4
        );
    }
}
