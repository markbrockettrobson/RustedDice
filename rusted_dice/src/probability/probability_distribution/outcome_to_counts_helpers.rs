use crate::{
    probability::{types::OutcomeToCountMap, ProbabilityOutcome},
    CountType,
};

use std::collections::btree_map::Entry::{Occupied, Vacant};

/// a helper function to add a [ProbabilityOutcome] to a [OutcomeToCountMap].
/// if the [ProbabilityOutcome] already exists in the [OutcomeToCountMap] the count will be added to the existing count.
///
/// # Arguments
///
/// * `outcome_to_count_map` - The [OutcomeToCountMap] to add the [ProbabilityOutcome] to.
/// * `probability_outcome` - The [ProbabilityOutcome] to add to the [OutcomeToCountMap].
/// * `count` - The count to add to the [ProbabilityOutcome] in the [OutcomeToCountMap].
///
/// # Example
/// ```
/// # use crate::rusted_dice::probability::ProbabilityOutcome;
/// # use crate::rusted_dice::probability::add_outcome_to_map;
/// # use std::collections::BTreeMap;
/// let mut outcome_to_count_map = BTreeMap::new();
/// let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(123);
/// let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(172);
///
/// add_outcome_to_map(&mut outcome_to_count_map, probability_outcome_one.clone(), 1);
/// add_outcome_to_map(&mut outcome_to_count_map, probability_outcome_two.clone(), 10);
///
/// assert!(outcome_to_count_map.get(&probability_outcome_one.clone()) == Some(&1));
/// assert!(outcome_to_count_map.get(&probability_outcome_two.clone()) == Some(&10));
///
/// add_outcome_to_map(&mut outcome_to_count_map, probability_outcome_one.clone(), 20);
///
/// assert!(outcome_to_count_map.get(&probability_outcome_one) == Some(&21));
/// assert!(outcome_to_count_map.get(&probability_outcome_two) == Some(&10));
/// ```
pub fn add_outcome_to_map(
    outcome_to_count_map: &mut OutcomeToCountMap,
    probability_outcome: ProbabilityOutcome,
    count: CountType,
) {
    match outcome_to_count_map.entry(probability_outcome) {
        Occupied(mut entry) => {
            *entry.get_mut() += count;
        }
        Vacant(entry) => {
            entry.insert(count);
        }
    };
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::probability::{add_outcome_to_map, ProbabilityOutcome};

    #[test]
    fn add_outcome_to_map_empty() {
        let mut outcome_to_count_map = BTreeMap::new();
        let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(123);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(172);

        add_outcome_to_map(
            &mut outcome_to_count_map,
            probability_outcome_one.clone(),
            1,
        );
        add_outcome_to_map(
            &mut outcome_to_count_map,
            probability_outcome_two.clone(),
            10,
        );

        assert!(outcome_to_count_map.get(&probability_outcome_one.clone()) == Some(&1));
        assert!(outcome_to_count_map.get(&probability_outcome_two.clone()) == Some(&10));
    }

    #[test]
    fn add_outcome_to_map_non_overlapping() {
        let mut outcome_to_count_map = BTreeMap::new();
        let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(123);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(172);
        outcome_to_count_map.insert(probability_outcome_one.clone(), 10);

        add_outcome_to_map(
            &mut outcome_to_count_map,
            probability_outcome_two.clone(),
            10,
        );

        assert!(outcome_to_count_map.get(&probability_outcome_one.clone()) == Some(&10));
        assert!(outcome_to_count_map.get(&probability_outcome_two.clone()) == Some(&10));
    }
    #[test]
    fn add_outcome_to_map_overlapping() {
        let mut outcome_to_count_map = BTreeMap::new();
        let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(123);
        outcome_to_count_map.insert(probability_outcome_one.clone(), 10);

        add_outcome_to_map(
            &mut outcome_to_count_map,
            probability_outcome_one.clone(),
            1,
        );

        assert!(outcome_to_count_map.get(&probability_outcome_one.clone()) == Some(&11));

        add_outcome_to_map(
            &mut outcome_to_count_map,
            probability_outcome_one.clone(),
            20,
        );

        assert!(outcome_to_count_map.get(&probability_outcome_one) == Some(&31));
    }
}
