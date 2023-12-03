use std::cmp::Ordering;

use prettytable::Table;

use crate::probability::ProbabilityDistribution;

use super::ToHashMap;

/// A trait for probability distributions to be turned into a Table
pub trait ToTable {
    fn to_table(&self) -> Table;
}

impl ToTable for ProbabilityDistribution {
    /// converts a [ProbabilityDistribution] into a Table (from the prettytable crate)
    ///
    /// # Arguments
    /// * `self` - the [ProbabilityDistribution] to convert
    ///
    /// # Returns
    /// * a Table with the following columns:
    ///  * value: the value of the outcome
    ///  * count: the number of times the outcome was observed
    ///  * constraint_name: the values of the constraint
    ///
    /// # Example
    /// ```
    /// # extern crate prettytable;
    /// # use std::collections::BTreeMap;
    /// # use std::collections::HashMap;
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::probability_distribution::ToHashMap;
    /// # use crate::rusted_dice::probability::ProbabilityOutcome;
    /// # use crate::rusted_dice::probability::add_outcome_to_map;
    /// # use crate::rusted_dice::constraint_management::Constraint;
    /// # use crate::rusted_dice::probability::probability_distribution::ToTable;
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
    /// b_tree_map.insert(ProbabilityOutcome::new_with_empty_constraint_map(98766), 1);
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
    /// let table = ProbabilityDistribution{outcome_counts: b_tree_map}.to_table();
    /// let out = "\
    /// +-------+-------+---------+---------+---+\n\
    /// | value | count | 1       | 8       | 9 |\n\
    /// +=======+=======+=========+=========+===+\n\
    /// | 12345 | 67890 | 3       |         |   |\n\
    /// +-------+-------+---------+---------+---+\n\
    /// | 12354 | 2     | 3, 4, 5 | 1, 2, 3 |   |\n\
    /// +-------+-------+---------+---------+---+\n\
    /// | 55555 | 66666 |         |         | 4 |\n\
    /// +-------+-------+---------+---------+---+\n\
    /// | 98766 | 1     |         |         |   |\n\
    /// +-------+-------+---------+---------+---+\n\
    /// ";
    /// assert_eq!(table.to_string().replace("\r\n", "\n"), out);
    /// ```
    fn to_table(&self) -> Table {
        let hash_map = self.to_hash_map();

        let mut table = Table::new();
        let mut column_names = hash_map.keys().collect::<Vec<&String>>();

        column_names.sort_by(|a, b| {
            if a == &"value" {
                Ordering::Less
            } else if b == &"value" {
                Ordering::Greater
            } else if a == &"count" {
                Ordering::Less
            } else if b == &"count" {
                Ordering::Greater
            } else {
                a.cmp(b)
            }
        });

        table.set_titles(
            column_names
                .clone()
                .into_iter()
                .map(|x| x.to_string())
                .collect(),
        );

        let columns: Vec<Vec<Option<String>>> = column_names
            .iter()
            .map(|column_name| hash_map.get(*column_name).unwrap().clone())
            .collect();

        for i in 0..columns[0].len() {
            let mut row = Vec::new();
            for column in &columns {
                row.push(column[i].clone().unwrap_or_default());
            }
            table.add_row(row.into());
        }
        table
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::constraint_management::Constraint;
    use crate::probability::probability_distribution::probability_distribution_to_table::ToTable;
    use crate::probability::{ProbabilityDistribution, ProbabilityOutcome};

    #[test]
    fn to_table_empty() {
        let table = ProbabilityDistribution::new_empty_distribution().to_table();

        let out = "\
        +-------+-------+\n\
        | value | count |\n\
        +=======+=======+\n\
        +-------+-------+\n\
        ";
        assert_eq!(table.to_string().replace("\r\n", "\n"), out);
    }

    #[test]
    fn to_table_no_constraints() {
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

        let table = ProbabilityDistribution {
            outcome_counts: b_tree_map,
        }
        .to_table();

        let out = "\
        +-------+-------+\n\
        | value | count |\n\
        +=======+=======+\n\
        | 12345 | 67890 |\n\
        +-------+-------+\n\
        | 12354 | 2     |\n\
        +-------+-------+\n\
        | 55555 | 66666 |\n\
        +-------+-------+\n\
        | 98766 | 1     |\n\
        +-------+-------+\n\
        ";
        assert_eq!(table.to_string().replace("\r\n", "\n"), out);
    }

    #[test]
    fn to_table_single_example_of_constraint() {
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

        let table = ProbabilityDistribution {
            outcome_counts: b_tree_map,
        }
        .to_table();
        let out = "\
        +-------+-------+-----+\n\
        | value | count | 123 |\n\
        +=======+=======+=====+\n\
        | 1000  | 10    | 1   |\n\
        +-------+-------+-----+\n\
        | 2000  | 20    |     |\n\
        +-------+-------+-----+\n\
        | 3000  | 30    |     |\n\
        +-------+-------+-----+\n\
        | 4000  | 40    |     |\n\
        +-------+-------+-----+\n\
        | 5000  | 50    | 5   |\n\
        +-------+-------+-----+\n\
        | 6000  | 60    |     |\n\
        +-------+-------+-----+\n\
        | 7000  | 70    | 7   |\n\
        +-------+-------+-----+\n\
        ";
        assert_eq!(table.to_string().replace("\r\n", "\n"), out);
    }

    #[test]
    fn to_table_many_example_of_single_constraint() {
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

        let table = ProbabilityDistribution {
            outcome_counts: b_tree_map,
        }
        .to_table();
        let out = "\
        +-------+-------+---------+\n\
        | value | count | 123     |\n\
        +=======+=======+=========+\n\
        | 12345 | 67890 | 3       |\n\
        +-------+-------+---------+\n\
        | 12354 | 2     | 1, 2, 3 |\n\
        +-------+-------+---------+\n\
        | 55555 | 66666 | 4       |\n\
        +-------+-------+---------+\n\
        | 98766 | 1     |         |\n\
        +-------+-------+---------+\n\
        ";
        assert_eq!(table.to_string().replace("\r\n", "\n"), out);
    }

    #[test]
    fn to_table_many_constraints() {
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

        let table = ProbabilityDistribution {
            outcome_counts: b_tree_map,
        }
        .to_table();
        let out = "\
        +-------+-------+---------+---------+---+\n\
        | value | count | 1       | 8       | 9 |\n\
        +=======+=======+=========+=========+===+\n\
        | 12345 | 67890 | 3       |         |   |\n\
        +-------+-------+---------+---------+---+\n\
        | 12354 | 2     | 3, 4, 5 | 1, 2, 3 |   |\n\
        +-------+-------+---------+---------+---+\n\
        | 55555 | 66666 |         |         | 4 |\n\
        +-------+-------+---------+---------+---+\n\
        | 98766 | 1     |         |         |   |\n\
        +-------+-------+---------+---------+---+\n\
        ";
        assert_eq!(table.to_string().replace("\r\n", "\n"), out);
    }
}
