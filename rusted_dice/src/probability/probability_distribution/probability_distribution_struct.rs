use std::collections::BTreeMap;

use crate::{probability::ProbabilityOutcome, types::CountType};

/// Represents a [ProbabilityDistribution].
///
/// Each [ProbabilityDistribution] has a map of [ProbabilityOutcome] ('outcome_counts') to the number of ways to creat that outcome.
///
/// # Examples
/// #### A [ProbabilityDistribution] with no [ProbabilityOutcome]s
/// ```
/// # use crate::rusted_dice::probability::ProbabilityDistribution;
/// let probability_distribution = ProbabilityDistribution::new_empty_distribution();
/// assert_eq!(probability_distribution.outcome_counts.len(), 0);
/// ```
///
/// ### A [ProbabilityDistribution] with a single [ProbabilityOutcome] and count one 1.
/// ```
/// # use crate::rusted_dice::probability::ProbabilityDistribution;
/// # use crate::rusted_dice::probability::ProbabilityOutcome;
/// let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(123);
/// let probability_distribution = ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome.clone());
/// assert!(probability_distribution.outcome_counts.get(&probability_outcome) == Some(&1));
/// ```
///
/// ### A [ProbabilityDistribution] with a many [ProbabilityOutcome]s.
/// the count will be 1 for all [ProbabilityOutcome]s. unless there are duplicates,
/// in which case the count will be the number of duplicates.
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
#[allow(dead_code)]
#[derive(Debug)]
pub struct ProbabilityDistribution {
    pub outcome_counts: BTreeMap<ProbabilityOutcome, CountType>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome_counts_set() {
        let test_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(12345);
        let test_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(55555);

        let mut b_tree_map = BTreeMap::new();
        b_tree_map.insert(test_outcome_one.clone(), 67890);
        b_tree_map.insert(test_outcome_two.clone(), 66666);

        let result = ProbabilityDistribution {
            outcome_counts: b_tree_map,
        };
        assert!(result.outcome_counts.get(&test_outcome_one) == Some(&67890));
        assert!(result.outcome_counts.get(&test_outcome_two) == Some(&66666));
    }
    #[test]
    fn test_fmt() {
        let test_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(1111);
        let test_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(2222);
        let test_outcome_three = ProbabilityOutcome::new_with_empty_constraint_map(3333);

        let mut b_tree_map = BTreeMap::new();
        b_tree_map.insert(test_outcome_one.clone(), 1);
        b_tree_map.insert(test_outcome_two.clone(), 2);
        b_tree_map.insert(test_outcome_three.clone(), 3);

        let result = ProbabilityDistribution {
            outcome_counts: b_tree_map,
        };

        assert_eq!(
            format!("{result:?}"),
            format!(
                "ProbabilityDistribution {{ outcome_counts: {{{:?}: 1, {:?}: 2, {:?}: 3}} }}",
                test_outcome_one, test_outcome_two, test_outcome_three
            )
        );
    }
}
