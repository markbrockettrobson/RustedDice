use std::collections::BTreeMap;

// use crate::constraint_management::{Constraint, ConstraintMap};
use crate::probability::ProbabilityDistribution;
use crate::probability::ProbabilityOutcome;
use crate::types::CountType;

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
    ///
    /// # Arguments
    ///
    /// * `probability_outcome` - The [ProbabilityOutcome] to add to the [ProbabilityDistribution].
    /// * `count` - The number of times to add the [ProbabilityOutcome] to the [ProbabilityDistribution].
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
    /// let probability_distribution = ProbabilityDistribution::new_single_probability_outcome(probability_outcome.clone(), 345);
    /// assert!(probability_distribution.outcome_counts.get(&probability_outcome) == Some(&345));
    /// ```
    pub fn new_single_probability_outcome(
        probability_outcome: ProbabilityOutcome,
        count: CountType,
    ) -> ProbabilityDistribution {
        let mut map = BTreeMap::new();
        map.insert(probability_outcome, count);
        ProbabilityDistribution {
            outcome_counts: map,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        probability::{ProbabilityDistribution, ProbabilityOutcome},
        types::CountType,
    };
    use proptest::prelude::*;

    #[test]
    fn test_new_empty_distribution() {
        let probability_distribution = ProbabilityDistribution::new_empty_distribution();
        assert_eq!(probability_distribution.outcome_counts.len(), 0);
    }
    proptest! {
        #[test]
        fn test_new_single_probability_outcome(count: CountType) {
            let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(123);
            let probability_distribution = ProbabilityDistribution::new_single_probability_outcome(probability_outcome.clone(), count);
            assert!(probability_distribution.outcome_counts.get(&probability_outcome) == Some(&count));
        }
    }
}
