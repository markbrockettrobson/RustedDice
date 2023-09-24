use std::collections::HashMap;

use polars::prelude::DataFrame;
use polars::prelude::NamedFrom;
use polars::prelude::Series;

use crate::constraint_management::ConstraintIdType;
use crate::probability::ProbabilityDistribution;
use crate::probability::ToDataFrame;
use crate::CountType;
use crate::ValueType;

impl ToDataFrame for ProbabilityDistribution {
    /// converts a [ProbabilityDistribution] into a polars [DataFrame]
    ///
    /// # Arguments
    /// * `self` - the [ProbabilityDistribution] to convert
    ///
    /// # Returns
    /// * a [DataFrame] with the following columns:
    ///  * value: the value of the outcome
    ///  * count: the number of times the outcome was observed
    ///  * constraint_name: the name of the constraint
    ///  * constraint_value: the value of the constraint
    ///
    /// # Example
    /// ```
    /// # use std::collections::BTreeMap;
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ProbabilityOutcome;
    /// # use crate::rusted_dice::probability::add_outcome_to_map;
    /// # use crate::rusted_dice::probability::ToDataFrame;
    /// # use crate::rusted_dice::constraint_management::Constraint;
    /// # use polars::frame::DataFrame;
    /// # use polars::series::Series;
    /// # use polars::prelude::NamedFrom;
    ///
    /// let mut b_tree_map = BTreeMap::new();
    /// b_tree_map.insert(
    ///     ProbabilityOutcome::new_with_constraints(
    ///         12345,
    ///         vec![Constraint::new_single_valid_value_constraint(123, 3)],
    ///     ),
    ///     67890,
    /// );
    /// b_tree_map.insert(
    ///     ProbabilityOutcome::new_with_empty_constraint_map(55555),
    ///     66666,
    /// );
    /// b_tree_map.insert(ProbabilityOutcome::new_with_empty_constraint_map(98766), 1);
    /// b_tree_map.insert(ProbabilityOutcome::new_with_empty_constraint_map(12354), 2);
    /// let result = ProbabilityDistribution {
    ///     outcome_counts: b_tree_map,
    /// }
    /// .to_dataframe();
    /// let df: DataFrame = DataFrame::new(vec![
    ///     Series::new("value", &[12345, 55555, 98766, 12354]),
    ///     Series::new("count", &[67890, 66666, 1, 2]),
    ///     Series::new("123", &[Some("3"), None, None, None]),
    /// ])
    /// .unwrap();
    /// assert_eq!(result, df);
    /// ```
    fn to_dataframe(&self) -> DataFrame {
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
                    .map(|&value| value)
                    .collect::<Vec<ValueType>>();
                    
                values.sort_by(|a, b| a.cmp(b));
                
                
                let value_string = values
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");

                match constraint_map_columns.get_mut(constraint_name) {
                    Some(column) => {
                        for _ in column.len()..index - 1 {
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

        let mut series = vec![
            Series::new("value", value_column),
            Series::new("count", count_column),
        ];

        let mut constraint_map_series = constraint_map_columns
            .iter_mut()
            .map(|(constraint_name, column)| {
                Series::new(constraint_name.to_string().as_str(), column)
            })
            .collect::<Vec<Series>>();
        
        constraint_map_series.sort_by(
            |a, b| a.name().cmp(b.name())
        );

        series.append(
            &mut constraint_map_series
        );

        let df = DataFrame::new(series).unwrap();
        df.sort(&["value"], vec![false], false).unwrap()
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
            Series::new("value", &[12345, 12354, 55555, 98766]),
            Series::new("count", &[67890, 2, 66666, 1]),
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
            Series::new("value", &[12345, 12354, 55555, 98766]),
            Series::new("count", &[67890, 2, 66666, 1]),
            Series::new("123", &[Some("3"), None, None, None]),
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
            Series::new("value", &[12345, 12354, 55555, 98766]),
            Series::new("count", &[67890, 2, 66666, 1]),
            Series::new("123", &[Some("3"), Some("1, 2, 3"), Some("4"), None]),
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
        b_tree_map.insert(
            ProbabilityOutcome::new_with_empty_constraint_map(98766), 1);
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
        .to_dataframe();

        let df: DataFrame = DataFrame::new(vec![
            Series::new("value", &[12345, 12354, 55555, 98766]),
            Series::new("count", &[67890, 2, 66666, 1]),
            Series::new("1", &[Some("3"), Some("3, 4, 5"), None, None]),
            Series::new("8", &[None, Some("1, 2, 3"), None, None]),
            Series::new("9", &[None, None, Some("4"), None]),
        ])
        .unwrap();

        assert_eq!(result, df);
    }
}
