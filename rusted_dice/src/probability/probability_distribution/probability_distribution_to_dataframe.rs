use polars::{
    prelude::{DataFrame, NamedFrom},
    series::Series,
};
use std::collections::hash_map::Entry::{Occupied, Vacant};

use crate::probability::ProbabilityDistribution;
use crate::probability::ToDataFrame;
use crate::ValueType;
use crate::{constraint_management::ConstraintIdType, CountType};

impl ToDataFrame for ProbabilityDistribution {
    fn to_dataframe(&self) -> DataFrame {
        let mut value_column: Vec<ValueType> = Vec::with_capacity(self.outcome_counts.len());
        let mut count_column: Vec<CountType> = Vec::with_capacity(self.outcome_counts.len());
        let mut constraint_columns: Vec<Vec<Option<Vec<ValueType>>>> = Vec::new();

        let mut constraint_columns_headings: Vec<ConstraintIdType> = Vec::new();

        for (outcome, _) in &self.outcome_counts {
            for (constraint_key, _) in &outcome.constraint_map.map {
                if !constraint_columns_headings.contains(&constraint_key) {
                    constraint_columns_headings.push(*constraint_key);
                    constraint_columns.push(Vec::new());
                }
            }
        }

        for (outcome, count) in &self.outcome_counts {
            value_column.push(outcome.value);
            count_column.push(*count);
            for index in 0..constraint_columns_headings.len() {
                if outcome
                    .constraint_map
                    .map
                    .contains_key(&constraint_columns_headings[index])
                {
                    constraint_columns[index]
                        .push(
                            Some(outcome
                                .constraint_map
                                .map
                                .get(&constraint_columns_headings[index])
                                .unwrap()
                                .valid_values
                                .clone()
                                .into_iter()
                                .collect()
                            )
                        );
                } else {
                    constraint_columns[index].push(None);
                }
            }
        }

        let series = &constraint_columns_headings
        .into_iter()
        .zip(constraint_columns.into_iter())
        .map(|(heading, column)| {
            Series::new(
                &heading.to_string(),
                &column
            )
        });

        DataFrame::new(
            vec![
                Series::new("value", value_column),
                Series::new("count", count_column),
            ].append(series)
        ).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use polars::datatypes::DataType::Int32;
    use polars::{
        prelude::{DataFrame, NamedFrom},
        series::Series,
    };

    use crate::constraint_management::Constraint;
    use crate::probability::{ProbabilityDistribution, ProbabilityOutcome, ToDataFrame};

    #[test]
    fn to_dataframe_empty() {
        let result = ProbabilityDistribution::new_empty_distribution().to_dataframe();

        let df: DataFrame = DataFrame::new(vec![
            Series::new_empty("value", &Int32),
            Series::new_empty("count", &Int32),
        ])
        .unwrap();

        assert_eq!(result, df);
    }

    #[test]
    fn to_dataframe_no_constraints() {
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
        .to_dataframe();

        let df: DataFrame = DataFrame::new(vec![
            Series::new("value", &[12345, 55555, 98766, 12354]),
            Series::new("count", &[67890, 66666, 1, 2]),
        ])
        .unwrap();

        assert_eq!(result, df);
    }

    #[test]
    fn to_dataframe_single_example_of_constraint() {
        let mut b_tree_map = BTreeMap::new();
        b_tree_map.insert(
            ProbabilityOutcome::new_with_constraints(
                12345,
                vec![Constraint::new_single_valid_value_constraint(123, 3)],
            ),
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
        .to_dataframe();

        let df: DataFrame = DataFrame::new(vec![
            Series::new("value", &[12345, 55555, 98766, 12354]),
            Series::new("count", &[67890, 66666, 1, 2]),
            Series::new("123", &[Some(3), None, None, None]),
        ])
        .unwrap();

        assert_eq!(result, df);
    }

    #[test]
    fn to_dataframe_many_example_of_single_constraint() {
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
        .to_dataframe();

        let df: DataFrame = DataFrame::new(vec![
            Series::new("value", &[12345, 55555, 98766, 12354]),
            Series::new("count", &[67890, 66666, 1, 2]),
            Series::new(
                "123",
                &[Some(vec![3]), Some(vec![4]), None, Some(vec![1, 2, 3])],
            ),
        ])
        .unwrap();

        assert_eq!(result, df);
    }

    #[test]
    fn to_dataframe_many_constraints() {
        let mut b_tree_map = BTreeMap::new();
        b_tree_map.insert(
            ProbabilityOutcome::new_with_constraints(
                12345,
                vec![Constraint::new_single_valid_value_constraint(3, 3)],
            ),
            67890,
        );
        b_tree_map.insert(
            ProbabilityOutcome::new_with_constraints(
                55555,
                vec![Constraint::new_single_valid_value_constraint(2, 4)],
            ),
            66666,
        );
        b_tree_map.insert(ProbabilityOutcome::new_with_empty_constraint_map(98766), 1);
        b_tree_map.insert(
            ProbabilityOutcome::new_with_constraints(
                12354,
                vec![Constraint::new_many_item_constraint(1, vec![1, 2, 3])],
            ),
            2,
        );

        let result = ProbabilityDistribution {
            outcome_counts: b_tree_map,
        }
        .to_dataframe();

        let df: DataFrame = DataFrame::new(vec![
            Series::new("value", &[12345, 55555, 98766, 12354]),
            Series::new("count", &[67890, 66666, 1, 2]),
            Series::new("1", &[None, None, None, Some(vec![1, 2, 3])]),
            Series::new("2", &[None, Some(vec![4]), None, None]),
            Series::new("3", &[Some(vec![3]), None, None, None]),
        ])
        .unwrap();

        assert_eq!(result, df);
    }
}
