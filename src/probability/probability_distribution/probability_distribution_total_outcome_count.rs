use crate::probability::ProbabilityDistribution;
use crate::CountType;

impl ProbabilityDistribution {
    /// Returns the total number of outcomes in the [ProbabilityDistribution].
    ///
    /// # Arguments
    ///
    /// * `self` - The [ProbabilityDistribution] to get the total number of outcomes from.
    ///
    /// # Returns
    ///
    /// Returns the total number of outcomes in the [ProbabilityDistribution] as a [CountType].
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ProbabilityOutcome;
    /// let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(1);
    /// let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(2);
    /// let probability_outcome_three = ProbabilityOutcome::new_with_empty_constraint_map(3);
    /// let probability_distribution = ProbabilityDistribution::new_from_many_probability_outcomes(
    ///     [
    ///         vec![probability_outcome_one.clone(); 123],
    ///         vec![probability_outcome_two.clone(); 567],
    ///         vec![probability_outcome_three.clone(); 111]
    ///     ].concat()
    /// );
    /// assert_eq!(probability_distribution.total_outcome_count(), 123 + 567 + 111);
    /// ```
    pub fn total_outcome_count(&self) -> CountType {
        self.outcome_counts.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::probability::ProbabilityOutcome;

    use super::*;

    #[test]
    fn test_empty() {
        let probability_distribution = ProbabilityDistribution::new_empty_distribution();
        assert_eq!(probability_distribution.total_outcome_count(), 0);
    }

    #[test]
    fn test_single_outcome() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(123);
        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);
        assert_eq!(probability_distribution.total_outcome_count(), 1);
    }

    #[test]
    fn test_many_of_the_same_outcome() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(123);
        let probability_distribution =
            ProbabilityDistribution::new_from_many_probability_outcomes(vec![
                probability_outcome
                    .clone();
                123
            ]);
        assert_eq!(probability_distribution.total_outcome_count(), 123);
    }

    #[test]
    fn test_many_different_outcomes() {
        let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(1);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(2);
        let probability_outcome_three = ProbabilityOutcome::new_with_empty_constraint_map(3);
        let probability_distribution = ProbabilityDistribution::new_from_many_probability_outcomes(
            [
                vec![probability_outcome_one.clone(); 123],
                vec![probability_outcome_two.clone(); 567],
                vec![probability_outcome_three.clone(); 111],
            ]
            .concat(),
        );
        assert_eq!(
            probability_distribution.total_outcome_count(),
            123 + 567 + 111
        );
    }
}
