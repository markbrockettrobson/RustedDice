use std::collections::BTreeMap;

use crate::probability::ProbabilityOutcome;

#[allow(dead_code)]
#[derive(Debug)]
pub struct ProbabilityDistribution {
    pub outcome_counts: BTreeMap<ProbabilityOutcome, u64>,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::constraint_management::constraint_map::ConstraintMap;

    use super::*;

    #[test]
    fn outcome_counts_set() {
        let test_outcome = ProbabilityOutcome {
            value: 12345,
            constraint_map: ConstraintMap {
                map: HashMap::new(),
            },
        };

        let mut b_tree_map = BTreeMap::new();
        b_tree_map.insert(test_outcome.clone(), 67890);

        let result = ProbabilityDistribution {
            outcome_counts: b_tree_map,
        };
        assert!(result.outcome_counts.get(&test_outcome) == Some(&67890));
    }
    #[test]
    fn test_fmt() {
        let test_outcome_one = ProbabilityOutcome {
            value: 1111,
            constraint_map: ConstraintMap {
                map: HashMap::new(),
            },
        };
        let test_outcome_two = ProbabilityOutcome {
            value: 2222,
            constraint_map: ConstraintMap {
                map: HashMap::new(),
            },
        };
        let test_outcome_three = ProbabilityOutcome {
            value: 3333,
            constraint_map: ConstraintMap {
                map: HashMap::new(),
            },
        };

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
