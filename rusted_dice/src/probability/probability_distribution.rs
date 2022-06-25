use std::collections::BTreeMap;

use super::probability_outcome::ProbabilityOutcome;

#[allow(dead_code)]
struct ProbabilityDistribution {
    pub outcome_counts: BTreeMap<ProbabilityOutcome, u64>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome_counts_set() {
        let test_outcome = ProbabilityOutcome {value: 12345};

        let mut b_tree_map = BTreeMap::new();
        b_tree_map.insert(test_outcome, 67890);

        let result = ProbabilityDistribution {outcome_counts: b_tree_map};
        assert!(result.outcome_counts.get(&test_outcome) == Some(&67890));
    }
}