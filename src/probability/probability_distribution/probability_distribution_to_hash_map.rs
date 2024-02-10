use std::collections::HashMap;

use crate::constraint_management::ConstraintIdType;
use crate::probability::ProbabilityDistribution;
use crate::CountType;
use crate::ValueType;

/// A trait for probability distributions to be turned into a Table
pub trait ToHashMap {
    fn to_hash_map(&self) -> HashMap<String, Vec<Option<String>>>;
}

impl ToHashMap for ProbabilityDistribution {
    /// converts a [ProbabilityDistribution] into a [HashMap] <String, [`Vec<Option<String>>`]>
    ///
    /// # Arguments
    /// * `self` - the [ProbabilityDistribution] to convert
    ///
    /// # Returns
    /// * a [HashMap] <String, `Vec<Option<String>>>` with the following columns:
    ///  * value: the value of the outcome
    ///  * count: the number of times the outcome was observed
    ///  * constraint_name: the values of the constraint
    ///
    /// # Example
    /// ```
    /// # use std::collections::BTreeMap;
    /// # use std::collections::HashMap;
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::probability_distribution::ToHashMap;
    /// # use crate::rusted_dice::probability::ProbabilityOutcome;
    /// # use crate::rusted_dice::probability::add_outcome_to_map;
    /// # use crate::rusted_dice::constraint_management::Constraint;
    ///
    /// let mut b_tree_map = BTreeMap::new();
    /// b_tree_map.insert(
    ///     ProbabilityOutcome::new_with_constraints(
    ///         12345,
    ///         vec![Constraint::new_single_valid_value_constraint(1, 3)],
    ///     ),
    ///     67890,
    /// );
    /// b_tree_map.insert(
    ///     ProbabilityOutcome::new_with_constraints(
    ///         55555,
    ///         vec![Constraint::new_single_valid_value_constraint(9, 4)],
    ///     ),
    ///     66666,
    /// );
    /// b_tree_map.insert(
    ///     ProbabilityOutcome::new_with_empty_constraint_map(98766), 1);
    /// b_tree_map.insert(
    ///     ProbabilityOutcome::new_with_constraints(
    ///         12354,
    ///         vec![
    ///             Constraint::new_many_item_constraint(8, vec![3, 2, 1]),
    ///             Constraint::new_many_item_constraint(1, vec![3, 5, 4]),
    ///         ],
    ///     ),
    ///     2,
    /// );
    ///
    /// let result = ProbabilityDistribution {
    ///     outcome_counts: b_tree_map,
    /// }.to_hash_map();
    ///
    /// let mut map: HashMap<String, Vec<Option<String>>> = HashMap::new();
    /// map.insert("value".to_string(), vec![Some("12345".to_string()), Some("12354".to_string()), Some("55555".to_string()), Some("98766".to_string())]);
    /// map.insert("count".to_string(), vec![Some("67890".to_string()), Some("2".to_string()), Some("66666".to_string()), Some("1".to_string())]);
    /// map.insert("1".to_string(), vec![Some("3".to_string()), Some("3, 4, 5".to_string()), None, None]);
    /// map.insert("8".to_string(), vec![None, Some("1, 2, 3".to_string()), None, None]);
    /// map.insert("9".to_string(), vec![None, None, Some("4".to_string()), None]);
    /// assert_eq!(result, map);
    /// ```
    fn to_hash_map(&self) -> HashMap<String, Vec<Option<String>>> {
        let mut value_column: Vec<ValueType> = Vec::with_capacity(self.outcome_counts.len());
        let mut count_column: Vec<CountType> = Vec::with_capacity(self.outcome_counts.len());
        let mut constraint_map_columns: HashMap<ConstraintIdType, Vec<Option<String>>> =
            HashMap::new();

        for (index, (outcome, count)) in self.outcome_counts.iter().enumerate() {
            value_column.push(outcome.value);
            count_column.push(*count);

            for (constraint_name, constraint_value) in outcome.constraint_map.map.iter() {
                let mut values = constraint_value
                    .valid_values
                    .iter()
                    .copied()
                    .collect::<Vec<ValueType>>();

                values.sort();

                let value_string = values
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");

                match constraint_map_columns.get_mut(constraint_name) {
                    Some(column) => {
                        for _ in column.len()..index {
                            column.push(None);
                        }
                        column.push(Some(value_string));
                    }
                    None => {
                        let mut column: Vec<Option<String>> =
                            Vec::with_capacity(self.outcome_counts.len());

                        for _ in 0..index {
                            column.push(None);
                        }
                        column.push(Some(value_string));
                        constraint_map_columns.insert(*constraint_name, column);
                    }
                }
            }
        }

        for (_, column) in constraint_map_columns.iter_mut() {
            for _ in column.len()..value_column.len() {
                column.push(None);
            }
        }

        let mut map: HashMap<String, Vec<Option<String>>> = HashMap::new();

        map.insert(
            "value".to_string(),
            value_column
                .iter()
                .map(|value| Some(value.to_string()))
                .collect::<Vec<Option<String>>>(),
        );
        map.insert(
            "count".to_string(),
            count_column
                .iter()
                .map(|count| Some(count.to_string()))
                .collect::<Vec<Option<String>>>(),
        );

        let mut constraint_map_columns_: Vec<(String, Vec<Option<String>>)> =
            constraint_map_columns
                .iter()
                .map(|(constraint_name, column)| (constraint_name.to_string(), column.to_owned()))
                .collect();

        constraint_map_columns_.sort_by(|a, b| a.0.cmp(&b.0));

        for (constraint_name, column) in constraint_map_columns.iter() {
            map.insert(constraint_name.to_string(), column.clone());
        }

        map
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::collections::HashMap;

    use crate::constraint_management::Constraint;
    use crate::probability::probability_distribution::probability_distribution_to_hash_map::ToHashMap;
    use crate::probability::{ProbabilityDistribution, ProbabilityOutcome};

    #[test]
    fn to_hash_map_empty() {
        let result = ProbabilityDistribution::new_empty_distribution().to_hash_map();

        let mut table: HashMap<String, Vec<Option<String>>> = HashMap::new();
        table.insert("value".to_string(), Vec::new());
        table.insert("count".to_string(), Vec::new());

        assert_eq!(result, table);
    }

    #[test]
    fn to_hash_map_no_constraints() {
        let mut b_tree_map = BTreeMap::new();
        b_tree_map.insert(
            ProbabilityOutcome::new_with_empty_constraint_map(12345),
            67890,
        );
        b_tree_map.insert(
            ProbabilityOutcome::new_with_empty_constraint_map(55555),
            66666,
        );
        b_tree_map.insert(ProbabilityOutcome::new_with_empty_constraint_map(98766), 1);
        b_tree_map.insert(ProbabilityOutcome::new_with_empty_constraint_map(12354), 2);

        let result = ProbabilityDistribution {
            outcome_counts: b_tree_map,
        }
        .to_hash_map();

        let mut table: HashMap<String, Vec<Option<String>>> = HashMap::new();
        table.insert(
            "value".to_string(),
            vec![
                Some("12345".to_string()),
                Some("12354".to_string()),
                Some("55555".to_string()),
                Some("98766".to_string()),
            ],
        );
        table.insert(
            "count".to_string(),
            vec![
                Some("67890".to_string()),
                Some("2".to_string()),
                Some("66666".to_string()),
                Some("1".to_string()),
            ],
        );

        assert_eq!(result, table);
    }

    #[test]
    fn to_hash_map_single_example_of_constraint() {
        let mut b_tree_map = BTreeMap::new();
        b_tree_map.insert(
            ProbabilityOutcome::new_with_constraints(
                1000,
                vec![Constraint::new_single_valid_value_constraint(123, 1)],
            ),
            10,
        );
        b_tree_map.insert(ProbabilityOutcome::new_with_empty_constraint_map(3000), 30);
        b_tree_map.insert(ProbabilityOutcome::new_with_empty_constraint_map(4000), 40);
        b_tree_map.insert(
            ProbabilityOutcome::new_with_constraints(
                5000,
                vec![Constraint::new_single_valid_value_constraint(123, 5)],
            ),
            50,
        );
        b_tree_map.insert(ProbabilityOutcome::new_with_empty_constraint_map(2000), 20);
        b_tree_map.insert(ProbabilityOutcome::new_with_empty_constraint_map(6000), 60);
        b_tree_map.insert(
            ProbabilityOutcome::new_with_constraints(
                7000,
                vec![Constraint::new_single_valid_value_constraint(123, 7)],
            ),
            70,
        );

        let result = ProbabilityDistribution {
            outcome_counts: b_tree_map,
        }
        .to_hash_map();

        let mut table: HashMap<String, Vec<Option<String>>> = HashMap::new();
        table.insert(
            "value".to_string(),
            vec![
                Some("1000".to_string()),
                Some("2000".to_string()),
                Some("3000".to_string()),
                Some("4000".to_string()),
                Some("5000".to_string()),
                Some("6000".to_string()),
                Some("7000".to_string()),
            ],
        );
        table.insert(
            "count".to_string(),
            vec![
                Some("10".to_string()),
                Some("20".to_string()),
                Some("30".to_string()),
                Some("40".to_string()),
                Some("50".to_string()),
                Some("60".to_string()),
                Some("70".to_string()),
            ],
        );
        table.insert(
            "123".to_string(),
            vec![
                Some("1".to_string()),
                None,
                None,
                None,
                Some("5".to_string()),
                None,
                Some("7".to_string()),
            ],
        );

        assert_eq!(result, table);
    }

    #[test]
    fn to_hash_map_many_example_of_single_constraint() {
        let mut b_tree_map = BTreeMap::new();
        b_tree_map.insert(
            ProbabilityOutcome::new_with_constraints(
                12345,
                vec![Constraint::new_single_valid_value_constraint(123, 3)],
            ),
            67890,
        );
        b_tree_map.insert(
            ProbabilityOutcome::new_with_constraints(
                55555,
                vec![Constraint::new_single_valid_value_constraint(123, 4)],
            ),
            66666,
        );
        b_tree_map.insert(ProbabilityOutcome::new_with_empty_constraint_map(98766), 1);
        b_tree_map.insert(
            ProbabilityOutcome::new_with_constraints(
                12354,
                vec![Constraint::new_many_item_constraint(123, vec![1, 2, 3])],
            ),
            2,
        );

        let result = ProbabilityDistribution {
            outcome_counts: b_tree_map,
        }
        .to_hash_map();

        let mut table: HashMap<String, Vec<Option<String>>> = HashMap::new();
        table.insert(
            "value".to_string(),
            vec![
                Some("12345".to_string()),
                Some("12354".to_string()),
                Some("55555".to_string()),
                Some("98766".to_string()),
            ],
        );
        table.insert(
            "count".to_string(),
            vec![
                Some("67890".to_string()),
                Some("2".to_string()),
                Some("66666".to_string()),
                Some("1".to_string()),
            ],
        );
        table.insert(
            "123".to_string(),
            vec![
                Some("3".to_string()),
                Some("1, 2, 3".to_string()),
                Some("4".to_string()),
                None,
            ],
        );

        assert_eq!(result, table);
    }

    #[test]
    fn to_hash_map_many_constraints() {
        let mut b_tree_map = BTreeMap::new();
        b_tree_map.insert(
            ProbabilityOutcome::new_with_constraints(
                12345,
                vec![Constraint::new_single_valid_value_constraint(1, 3)],
            ),
            67890,
        );
        b_tree_map.insert(
            ProbabilityOutcome::new_with_constraints(
                55555,
                vec![Constraint::new_single_valid_value_constraint(9, 4)],
            ),
            66666,
        );
        b_tree_map.insert(ProbabilityOutcome::new_with_empty_constraint_map(98766), 1);
        b_tree_map.insert(
            ProbabilityOutcome::new_with_constraints(
                12354,
                vec![
                    Constraint::new_many_item_constraint(8, vec![3, 2, 1]),
                    Constraint::new_many_item_constraint(1, vec![3, 5, 4]),
                ],
            ),
            2,
        );

        let result = ProbabilityDistribution {
            outcome_counts: b_tree_map,
        }
        .to_hash_map();

        let mut table: HashMap<String, Vec<Option<String>>> = HashMap::new();
        table.insert(
            "value".to_string(),
            vec![
                Some("12345".to_string()),
                Some("12354".to_string()),
                Some("55555".to_string()),
                Some("98766".to_string()),
            ],
        );
        table.insert(
            "count".to_string(),
            vec![
                Some("67890".to_string()),
                Some("2".to_string()),
                Some("66666".to_string()),
                Some("1".to_string()),
            ],
        );
        table.insert(
            "1".to_string(),
            vec![
                Some("3".to_string()),
                Some("3, 4, 5".to_string()),
                None,
                None,
            ],
        );
        table.insert(
            "8".to_string(),
            vec![None, Some("1, 2, 3".to_string()), None, None],
        );
        table.insert(
            "9".to_string(),
            vec![None, None, Some("4".to_string()), None],
        );

        assert_eq!(result, table);
    }
}
