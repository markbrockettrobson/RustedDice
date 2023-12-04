use std::collections::BTreeMap;
use std::ops::Add;

use crate::constraint_management::Constraint;
use crate::probability::ProbabilityDistribution;

impl Add<Constraint> for ProbabilityDistribution {
    type Output = Self;

    /// Implements the addition operator for [ProbabilityDistribution] and [Constraint].
    /// The [Constraint] is added to all of the ProbabilityOutcomes.
    ///
    /// # Arguments
    ///
    /// * `self` - The [ProbabilityDistribution] to add the constraint to.
    /// * `constraint` - The constraint to add to each outcome in the [ProbabilityDistribution].
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityDistribution] after the addition operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::constraint_management::Constraint;
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ProbabilityOutcome;
    /// # use std::collections::BTreeMap;
    /// let constraint_one = Constraint::new_many_item_constraint(24, vec![10,20,30,40,50,60]);
    /// let constraint_two = Constraint::new_many_item_constraint(24, vec![40,50,60,70,80,90]);
    /// let constraint_three = Constraint::new_many_item_constraint(24, vec![40,50,60]);
    ///
    /// let mut b_tree_map = BTreeMap::new();
    /// b_tree_map.insert(ProbabilityOutcome::new_with_constraints(1111, vec![constraint_one]), 99);
    /// let probability_distribution = ProbabilityDistribution{outcome_counts: b_tree_map};
    ///
    /// let probability_distribution_with_constraint = probability_distribution + constraint_two.clone();
    /// assert_eq!(
    ///     probability_distribution_with_constraint.outcome_counts.get(&ProbabilityOutcome::new_with_constraints(1111, vec![constraint_three])),
    ///     Some(&99)
    /// );
    /// ```
    fn add(self, constraint: Constraint) -> Self {
        let mut b_tree_map = BTreeMap::new();

        self.outcome_counts
            .iter()
            .map(|(outcome, count)| (outcome.clone() + constraint.clone(), count))
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
        let constraint = Constraint::new_single_valid_value_constraint(1, 1);
        let probability_distribution_with_constraint = probability_distribution + constraint;

        assert_eq!(
            probability_distribution_with_constraint
                .outcome_counts
                .len(),
            0
        );
    }

    #[test]
    fn test_single_outcome_without_constraint() {
        let constraint = Constraint::new_single_valid_value_constraint(24, 56);

        let mut b_tree_map = BTreeMap::new();
        b_tree_map.insert(ProbabilityOutcome::new_with_empty_constraint_map(1111), 99);
        let probability_distribution = ProbabilityDistribution {
            outcome_counts: b_tree_map,
        };

        let probability_distribution_with_constraint =
            probability_distribution + constraint.clone();

        assert_eq!(
            probability_distribution_with_constraint.outcome_counts.get(
                &ProbabilityOutcome::new_with_constraints(1111, vec![constraint])
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
        let constraint_one = Constraint::new_many_item_constraint(24, vec![10, 20, 30, 40, 50, 60]);
        let constraint_two = Constraint::new_many_item_constraint(24, vec![40, 50, 60, 70, 80, 90]);
        let constraint_three = Constraint::new_many_item_constraint(24, vec![40, 50, 60]);

        let mut b_tree_map = BTreeMap::new();
        b_tree_map.insert(
            ProbabilityOutcome::new_with_constraints(1111, vec![constraint_one]),
            99,
        );
        let probability_distribution = ProbabilityDistribution {
            outcome_counts: b_tree_map,
        };

        let probability_distribution_with_constraint =
            probability_distribution + constraint_two.clone();

        assert_eq!(
            probability_distribution_with_constraint.outcome_counts.get(
                &ProbabilityOutcome::new_with_constraints(1111, vec![constraint_three])
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
        let constraint_one = Constraint::new_many_item_constraint(24, vec![10, 20, 30, 40, 50, 60]);
        let constraint_two = Constraint::new_many_item_constraint(24, vec![40, 50, 60, 70, 80, 90]);
        let constraint_three = Constraint::new_many_item_constraint(24, vec![40, 50, 60]);

        let constraint_four = Constraint::new_many_item_constraint(240, vec![1, 2, 3]);

        let mut b_tree_map = BTreeMap::new();
        b_tree_map.insert(
            ProbabilityOutcome::new_with_constraints(
                1111,
                vec![constraint_one, constraint_four.clone()],
            ),
            99,
        );
        let probability_distribution = ProbabilityDistribution {
            outcome_counts: b_tree_map,
        };

        let probability_distribution_with_constraint =
            probability_distribution + constraint_two.clone();

        assert_eq!(
            probability_distribution_with_constraint.outcome_counts.get(
                &ProbabilityOutcome::new_with_constraints(
                    1111,
                    vec![constraint_three, constraint_four]
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
}
