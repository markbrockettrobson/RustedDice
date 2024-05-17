use std::collections::BTreeMap;

use crate::probability::ProbabilityDistribution;
use crate::probability::ProbabilityOutcome;
use crate::ValueType;

use super::add_outcome_to_map;

#[allow(dead_code)]
impl ProbabilityDistribution {
    /// Creates a new [ProbabilityDistribution] with no [ProbabilityOutcome]s.
    ///
    /// # Returns
    ///
    /// The new empty [ProbabilityDistribution].
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// let probability_distribution = ProbabilityDistribution::new_empty_distribution();
    /// assert_eq!(probability_distribution.outcome_counts.len(), 0);
    /// ```
    pub fn new_empty_distribution() -> ProbabilityDistribution {
        ProbabilityDistribution {
            outcome_counts: BTreeMap::new(),
        }
    }

    /// Creates a new [ProbabilityDistribution] with a single [ProbabilityOutcome].
    /// the count will be 1 for this [ProbabilityOutcome].
    ///
    /// # Arguments
    ///
    /// * `probability_outcome` - The [ProbabilityOutcome] to add to the [ProbabilityDistribution].
    ///
    /// # Returns
    ///
    /// The new [ProbabilityDistribution].
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ProbabilityOutcome;
    /// let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(123);
    /// let probability_distribution = ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome.clone());
    /// assert!(probability_distribution.outcome_counts.get(&probability_outcome) == Some(&1));
    /// ```
    pub fn new_from_single_probability_outcome(
        probability_outcome: ProbabilityOutcome,
    ) -> ProbabilityDistribution {
        let mut map = BTreeMap::new();
        add_outcome_to_map(&mut map, probability_outcome, 1);
        ProbabilityDistribution {
            outcome_counts: map,
        }
    }

    /// Creates a new [ProbabilityDistribution] with many [ProbabilityOutcome]s.
    /// the count will be 1 for all [ProbabilityOutcome]s.
    /// unless there are duplicates, in which case the count will be the number of duplicates.
    ///
    /// # Arguments
    ///
    /// * `probability_outcome` - The [ProbabilityOutcome] to add to the [ProbabilityDistribution].
    ///
    /// # Returns
    ///
    /// The new [ProbabilityDistribution].
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
    ///     vec![
    ///         probability_outcome_one.clone(),
    ///         probability_outcome_two.clone(),
    ///         probability_outcome_three.clone()
    ///     ]
    /// );
    /// assert_eq!(probability_distribution.outcome_counts.get(&probability_outcome_one), Some(&1));
    /// assert_eq!(probability_distribution.outcome_counts.get(&probability_outcome_two), Some(&1));
    /// assert_eq!(probability_distribution.outcome_counts.get(&probability_outcome_three), Some(&1));
    /// assert_eq!(probability_distribution.outcome_counts.len(), 3);
    /// ```
    pub fn new_from_many_probability_outcomes(
        probability_outcomes: impl IntoIterator<Item = ProbabilityOutcome>,
    ) -> ProbabilityDistribution {
        let mut map = BTreeMap::new();
        for probability_outcome in probability_outcomes {
            add_outcome_to_map(&mut map, probability_outcome, 1)
        }
        ProbabilityDistribution {
            outcome_counts: map,
        }
    }

    /// Creates a new [ProbabilityDistribution] with [ProbabilityOutcome]s representing a N sided dice.
    /// the count will be 1 for all [ProbabilityOutcome]s.
    /// for example, if n is 6, the [ProbabilityDistribution] will have 6 [ProbabilityOutcome]s.
    /// 1, 2, 3, 4, 5, 6.
    ///
    /// for negative n, the [ProbabilityDistribution] will have vales from -1 to -n.
    /// for example if n is -6, the [ProbabilityDistribution] will have 6 [ProbabilityOutcome]s.
    /// -1, -2, -3, -4, -5, -6.
    ///
    /// for Zero n, the [ProbabilityDistribution] will have no [ProbabilityOutcome]s.
    /// # Arguments
    ///
    /// * `number_of_sides` - [ValueType] The the number of sides the dice has.
    ///
    /// # Returns
    ///
    /// The new [ProbabilityDistribution].
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ToTable;
    /// let probability_distribution = ProbabilityDistribution::new_dice(5);
    ///
    /// assert_eq!(
    ///     probability_distribution
    ///         .to_table()
    ///         .to_string()
    ///         .replace("\r\n", "\n"),
    ///     "\
    ///     +-------+-------+\n\
    ///     | value | count |\n\
    ///     +=======+=======+\n\
    ///     | 1     | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 2     | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 3     | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 4     | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 5     | 1     |\n\
    ///     +-------+-------+\n\
    ///     "
    /// );
    /// ```
    pub fn new_dice(number_of_sides: ValueType) -> ProbabilityDistribution {
        let mut map = BTreeMap::new();
        for i in 1..number_of_sides.abs() + 1 {
            add_outcome_to_map(
                &mut map,
                ProbabilityOutcome::new_with_empty_constraint_map(
                    if number_of_sides.is_positive() { i } else { -i },
                ),
                1,
            )
        }
        ProbabilityDistribution {
            outcome_counts: map,
        }
    }

    /// Creates a new [ProbabilityDistribution] with [ProbabilityOutcome]s representing rolling M, N sided dice.
    /// for example, if m is 2 if n is 4, the [ProbabilityDistribution] will have 10 [ProbabilityOutcome]s.
    /// 3 to 12
    ///
    /// for negative n, the [ProbabilityDistribution] will have vales from -1 to -n.
    /// for example if n is -6, the [ProbabilityDistribution] will have 6 [ProbabilityOutcome]s.
    /// -1, -2, -3, -4, -5, -6.
    ///
    ///
    /// for Zero n, the [ProbabilityDistribution] will have no [ProbabilityOutcome]s.
    ///
    /// for Zero m, the [ProbabilityDistribution] will have no [ProbabilityOutcome]s.
    ///
    ///
    /// # Arguments
    ///
    /// * `number_of_dice` - [u16] The the number of dice to roll.
    /// * `number_of_sides` - [ValueType] The the number of sides the dice has.
    ///
    /// # Returns
    ///
    /// The new [ProbabilityDistribution].
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ToTable;
    ///let three_d4 = ProbabilityDistribution::new_multiple_dice(3, 4);
    ///
    ///assert_eq!(
    ///    three_d4
    ///        .to_table()
    ///        .to_string()
    ///        .replace("\r\n", "\n"),
    ///     "\
    ///     +-------+-------+\n\
    ///     | value | count |\n\
    ///     +=======+=======+\n\
    ///     | 3     | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 4     | 3     |\n\
    ///     +-------+-------+\n\
    ///     | 5     | 6     |\n\
    ///     +-------+-------+\n\
    ///     | 6     | 10    |\n\
    ///     +-------+-------+\n\
    ///     | 7     | 12    |\n\
    ///     +-------+-------+\n\
    ///     | 8     | 12    |\n\
    ///     +-------+-------+\n\
    ///     | 9     | 10    |\n\
    ///     +-------+-------+\n\
    ///     | 10    | 6     |\n\
    ///     +-------+-------+\n\
    ///     | 11    | 3     |\n\
    ///     +-------+-------+\n\
    ///     | 12    | 1     |\n\
    ///     +-------+-------+\n\
    ///     ");
    /// ```
    pub fn new_multiple_dice(
        number_of_dice: u16,
        number_of_sides: ValueType,
    ) -> ProbabilityDistribution {
        if number_of_dice == 0 || number_of_sides == 0 {
            return ProbabilityDistribution::new_empty_distribution();
        }

        let single_dice = ProbabilityDistribution::new_dice(number_of_sides);
        let mut combined_probability_distribution = single_dice.clone();

        for _ in 1..number_of_dice {
            combined_probability_distribution =
                combined_probability_distribution + single_dice.clone();
        }

        combined_probability_distribution
    }
}

#[cfg(test)]
mod tests {
    use crate::probability::{ProbabilityDistribution, ProbabilityOutcome, ToTable};

    #[test]
    fn test_new_empty_distribution() {
        let probability_distribution = ProbabilityDistribution::new_empty_distribution();
        assert_eq!(probability_distribution.outcome_counts.len(), 0);
    }

    #[test]
    fn test_new_from_single_probability_outcome() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(123);
        let probability_distribution = ProbabilityDistribution::new_from_single_probability_outcome(
            probability_outcome.clone(),
        );
        assert!(
            probability_distribution
                .outcome_counts
                .get(&probability_outcome)
                == Some(&1)
        );
    }

    #[test]
    fn test_new_from_many_probability_outcomes_empty() {
        let probability_distribution =
            ProbabilityDistribution::new_from_many_probability_outcomes(vec![]);
        assert_eq!(probability_distribution.outcome_counts.len(), 0);
    }

    #[test]
    fn test_new_from_many_probability_outcomes_single() {
        let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(1);
        let probability_distribution =
            ProbabilityDistribution::new_from_many_probability_outcomes(vec![
                probability_outcome_one.clone(),
            ]);
        assert_eq!(
            probability_distribution
                .outcome_counts
                .get(&probability_outcome_one),
            Some(&1)
        );
        assert_eq!(probability_distribution.outcome_counts.len(), 1);
    }

    #[test]
    fn test_new_from_many_probability_outcomes_many() {
        let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(1);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(2);
        let probability_outcome_three = ProbabilityOutcome::new_with_empty_constraint_map(3);
        let probability_distribution =
            ProbabilityDistribution::new_from_many_probability_outcomes(vec![
                probability_outcome_one.clone(),
                probability_outcome_two.clone(),
                probability_outcome_three.clone(),
            ]);
        assert_eq!(
            probability_distribution
                .outcome_counts
                .get(&probability_outcome_one),
            Some(&1)
        );
        assert_eq!(
            probability_distribution
                .outcome_counts
                .get(&probability_outcome_two),
            Some(&1)
        );
        assert_eq!(
            probability_distribution
                .outcome_counts
                .get(&probability_outcome_three),
            Some(&1)
        );
        assert_eq!(probability_distribution.outcome_counts.len(), 3);
    }

    #[test]
    fn test_new_from_many_probability_outcomes_many_duplicates() {
        let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(1);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(2);
        let probability_outcome_three = ProbabilityOutcome::new_with_empty_constraint_map(3);
        let probability_distribution =
            ProbabilityDistribution::new_from_many_probability_outcomes(vec![
                probability_outcome_one.clone(),
                probability_outcome_two.clone(),
                probability_outcome_two.clone(),
                probability_outcome_three.clone(),
                probability_outcome_three.clone(),
                probability_outcome_three.clone(),
            ]);
        assert_eq!(
            probability_distribution
                .outcome_counts
                .get(&probability_outcome_one),
            Some(&1)
        );
        assert_eq!(
            probability_distribution
                .outcome_counts
                .get(&probability_outcome_two),
            Some(&2)
        );
        assert_eq!(
            probability_distribution
                .outcome_counts
                .get(&probability_outcome_three),
            Some(&3)
        );
        assert_eq!(probability_distribution.outcome_counts.len(), 3);
    }

    #[test]
    fn test_new_dice_zero() {
        let probability_distribution = ProbabilityDistribution::new_dice(0);
        assert_eq!(probability_distribution.outcome_counts.len(), 0);
    }

    #[test]
    fn test_new_dice_five() {
        let probability_distribution = ProbabilityDistribution::new_dice(5);

        assert_eq!(
            probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
            +-------+-------+\n\
            | value | count |\n\
            +=======+=======+\n\
            | 1     | 1     |\n\
            +-------+-------+\n\
            | 2     | 1     |\n\
            +-------+-------+\n\
            | 3     | 1     |\n\
            +-------+-------+\n\
            | 4     | 1     |\n\
            +-------+-------+\n\
            | 5     | 1     |\n\
            +-------+-------+\n\
            "
        );
    }

    #[test]
    fn test_new_dice_negative() {
        let probability_distribution = ProbabilityDistribution::new_dice(-4);

        assert_eq!(
            probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
            +-------+-------+\n\
            | value | count |\n\
            +=======+=======+\n\
            | -4    | 1     |\n\
            +-------+-------+\n\
            | -3    | 1     |\n\
            +-------+-------+\n\
            | -2    | 1     |\n\
            +-------+-------+\n\
            | -1    | 1     |\n\
            +-------+-------+\n\
            "
        );
    }

    #[test]
    fn test_new_multiple_zero_dice_zero_sides() {
        let probability_distribution = ProbabilityDistribution::new_multiple_dice(0, 0);
        assert_eq!(probability_distribution.outcome_counts.len(), 0);
    }

    #[test]
    fn test_new_multiple_dice_zero_sides() {
        let probability_distribution = ProbabilityDistribution::new_multiple_dice(1, 0);
        assert_eq!(probability_distribution.outcome_counts.len(), 0);
    }

    #[test]
    fn test_new_multiple_dice_zero_dice() {
        let probability_distribution = ProbabilityDistribution::new_multiple_dice(0, 1);

        assert_eq!(
            probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
            +-------+-------+\n\
            | value | count |\n\
            +=======+=======+\n\
            +-------+-------+\n\
            "
        );
    }

    #[test]
    fn test_new_multiple_dice_one_dice_one_side() {
        let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(1);
        let probability_distribution = ProbabilityDistribution::new_multiple_dice(1, 1);
        assert_eq!(
            probability_distribution
                .outcome_counts
                .get(&probability_outcome_one),
            Some(&1)
        );
        assert_eq!(probability_distribution.outcome_counts.len(), 1);
    }

    #[test]
    fn test_new_multiple_dice_one_dice_five_sides() {
        let probability_distribution = ProbabilityDistribution::new_multiple_dice(1, 5);

        assert_eq!(
            probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
            +-------+-------+\n\
            | value | count |\n\
            +=======+=======+\n\
            | 1     | 1     |\n\
            +-------+-------+\n\
            | 2     | 1     |\n\
            +-------+-------+\n\
            | 3     | 1     |\n\
            +-------+-------+\n\
            | 4     | 1     |\n\
            +-------+-------+\n\
            | 5     | 1     |\n\
            +-------+-------+\n\
            "
        );
    }

    #[test]
    fn test_new_multiple_dice_two_dice_one_side() {
        let probability_distribution = ProbabilityDistribution::new_multiple_dice(2, 1);

        assert_eq!(
            probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
            +-------+-------+\n\
            | value | count |\n\
            +=======+=======+\n\
            | 2     | 1     |\n\
            +-------+-------+\n\
            "
        );

        assert_eq!(probability_distribution.outcome_counts.len(), 1);
    }

    #[test]
    fn test_new_multiple_dice_two_dice_five_sides() {
        let probability_distribution = ProbabilityDistribution::new_multiple_dice(2, 5);

        assert_eq!(
            probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
            +-------+-------+\n\
            | value | count |\n\
            +=======+=======+\n\
            | 2     | 1     |\n\
            +-------+-------+\n\
            | 3     | 2     |\n\
            +-------+-------+\n\
            | 4     | 3     |\n\
            +-------+-------+\n\
            | 5     | 4     |\n\
            +-------+-------+\n\
            | 6     | 5     |\n\
            +-------+-------+\n\
            | 7     | 4     |\n\
            +-------+-------+\n\
            | 8     | 3     |\n\
            +-------+-------+\n\
            | 9     | 2     |\n\
            +-------+-------+\n\
            | 10    | 1     |\n\
            +-------+-------+\n\
            "
        );
    }

    #[test]
    fn test_new_multiple_dice_three_dice_four_sides() {
        let probability_distribution = ProbabilityDistribution::new_multiple_dice(3, 4);

        assert_eq!(
            probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
            +-------+-------+\n\
            | value | count |\n\
            +=======+=======+\n\
            | 3     | 1     |\n\
            +-------+-------+\n\
            | 4     | 3     |\n\
            +-------+-------+\n\
            | 5     | 6     |\n\
            +-------+-------+\n\
            | 6     | 10    |\n\
            +-------+-------+\n\
            | 7     | 12    |\n\
            +-------+-------+\n\
            | 8     | 12    |\n\
            +-------+-------+\n\
            | 9     | 10    |\n\
            +-------+-------+\n\
            | 10    | 6     |\n\
            +-------+-------+\n\
            | 11    | 3     |\n\
            +-------+-------+\n\
            | 12    | 1     |\n\
            +-------+-------+\n\
            "
        );
    }

    #[test]
    fn test_new_multiple_dice_one_dice_negative_four_sides() {
        let probability_distribution = ProbabilityDistribution::new_multiple_dice(1, -4);

        assert_eq!(
            probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
            +-------+-------+\n\
            | value | count |\n\
            +=======+=======+\n\
            | -4    | 1     |\n\
            +-------+-------+\n\
            | -3    | 1     |\n\
            +-------+-------+\n\
            | -2    | 1     |\n\
            +-------+-------+\n\
            | -1    | 1     |\n\
            +-------+-------+\n\
            "
        );
    }

    #[test]
    fn test_new_multiple_dice_three_dice_negative_four_sides() {
        let probability_distribution = ProbabilityDistribution::new_multiple_dice(3, -4);

        assert_eq!(
            probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
            +-------+-------+\n\
            | value | count |\n\
            +=======+=======+\n\
            | -12   | 1     |\n\
            +-------+-------+\n\
            | -11   | 3     |\n\
            +-------+-------+\n\
            | -10   | 6     |\n\
            +-------+-------+\n\
            | -9    | 10    |\n\
            +-------+-------+\n\
            | -8    | 12    |\n\
            +-------+-------+\n\
            | -7    | 12    |\n\
            +-------+-------+\n\
            | -6    | 10    |\n\
            +-------+-------+\n\
            | -5    | 6     |\n\
            +-------+-------+\n\
            | -4    | 3     |\n\
            +-------+-------+\n\
            | -3    | 1     |\n\
            +-------+-------+\n\
            "
        );
    }
}
