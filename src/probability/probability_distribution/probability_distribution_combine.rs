use crate::{
    constraint_management::IsTheoreticallyPossible,
    probability::{BinaryOperation, Combine, ProbabilityDistribution, ProbabilityOutcome},
    CountType, ValueType,
};
use std::collections::BTreeMap;

use super::add_outcome_to_map;

impl Combine for ProbabilityDistribution {
    /// Combine this instance with another instance using the specified [BinaryOperation].
    /// in the order: self [BinaryOperation] `other`
    ///
    /// values are combined using the [BinaryOperation] function.
    /// constraint maps are combined using the ConstraintMap::add function.
    ///
    /// # Arguments
    ///
    /// * `other` - The self type to check preform the [BinaryOperation] with.
    /// * `binary_operation` - the [BinaryOperation] function.
    ///
    /// # Returns
    ///
    /// Returns the [ProbabilityDistribution] type result of the [BinaryOperation] function.
    fn combine(&self, other: Self, binary_operation: BinaryOperation) -> Self {
        let mut new_outcome_counts: BTreeMap<ProbabilityOutcome, CountType> = BTreeMap::new();

        for (value_one, count_one) in self.outcome_counts.iter() {
            for (value_two, count_two) in other.outcome_counts.iter() {
                let new_value = value_one.combine(value_two.clone(), binary_operation);
                if new_value.constraint_map.is_theoretically_possible() {
                    let new_count = *count_one * count_two;
                    add_outcome_to_map(&mut new_outcome_counts, new_value, new_count);
                }
            }
        }
        ProbabilityDistribution {
            outcome_counts: new_outcome_counts,
        }
    }

    /// Combine this instance with a [ValueType] using the specified [BinaryOperation].
    /// in the order: self [BinaryOperation] [ValueType]
    ///
    /// values are combined using the [BinaryOperation] function.
    /// constraint maps are combined using the ConstraintMap::add function.
    ///
    /// # Arguments
    ///
    /// * `other` - The [ValueType] to check preform the [BinaryOperation] with.
    /// * `binary_operation` - the [BinaryOperation] function.
    ///
    /// # Returns
    ///
    /// Returns the [ProbabilityDistribution] type result of the [BinaryOperation] function.
    fn combine_value_type(&self, other: ValueType, binary_operation: BinaryOperation) -> Self {
        let mut new_outcome_counts: BTreeMap<ProbabilityOutcome, CountType> = BTreeMap::new();

        for (value, count) in self.outcome_counts.iter() {
            let new_value = value.combine_value_type(other, binary_operation);
            let new_count = *count;
            add_outcome_to_map(&mut new_outcome_counts, new_value, new_count);
        }
        ProbabilityDistribution {
            outcome_counts: new_outcome_counts,
        }
    }

    /// Combine this instance with a [ValueType] using the specified [BinaryOperation].
    /// in the order: [ValueType] [BinaryOperation] self
    ///
    /// values are combined using the [BinaryOperation] function.
    /// constraint maps are combined using the ConstraintMap::add function.
    ///
    /// # Arguments
    ///
    /// * `other` - The [ValueType] to check preform the [BinaryOperation] with.
    /// * `binary_operation` - the [BinaryOperation] function.
    ///
    /// # Returns
    ///
    /// Returns the [ProbabilityDistribution] type result of the [BinaryOperation] function.
    fn value_type_combine(&self, other: ValueType, binary_operation: BinaryOperation) -> Self {
        let mut new_outcome_counts: BTreeMap<ProbabilityOutcome, CountType> = BTreeMap::new();

        for (value, count) in self.outcome_counts.iter() {
            let new_value = value.value_type_combine(other, binary_operation);
            let new_count = *count;
            add_outcome_to_map(&mut new_outcome_counts, new_value, new_count);
        }
        ProbabilityDistribution {
            outcome_counts: new_outcome_counts,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::constraint_management::Constraint;
    use crate::probability::probability_distribution::ToTable;
    use crate::probability::{Combine, ProbabilityDistribution, ProbabilityOutcome};

    // tests with empty constraint
    #[test]
    fn test_combine_empty_two_probability_distributions() {
        let probability_distribution_one = ProbabilityDistribution::new_empty_distribution();
        let probability_distribution_two = ProbabilityDistribution::new_empty_distribution();

        let combined_probability_distribution = probability_distribution_one
            .combine(probability_distribution_two, |lhs, rhs| lhs - rhs);

        let out = "\
        +-------+-------+\n\
        | value | count |\n\
        +=======+=======+\n\
        +-------+-------+\n\
        ";
        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            out
        );
    }

    #[test]
    fn test_combine_empty_probability_distribution_with_value_type() {
        let probability_distribution = ProbabilityDistribution::new_empty_distribution();

        let combined_probability_distribution =
            probability_distribution.combine_value_type(3, |lhs, rhs| lhs - rhs);

        let out = "\
        +-------+-------+\n\
        | value | count |\n\
        +=======+=======+\n\
        +-------+-------+\n\
        ";
        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            out
        );
    }

    #[test]
    fn test_combine_value_type_with_empty_probability_distribution() {
        let probability_distribution = ProbabilityDistribution::new_empty_distribution();

        let combined_probability_distribution =
            probability_distribution.value_type_combine(3, |lhs, rhs| lhs - rhs);

        let out = "\
        +-------+-------+\n\
        | value | count |\n\
        +=======+=======+\n\
        +-------+-------+\n\
        ";
        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            out
        );
    }

    #[test]
    fn test_combine_single_outcome_probability_distribution_with_empty_probability_distribution() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(123);
        let probability_distribution_one =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);
        let probability_distribution_two = ProbabilityDistribution::new_empty_distribution();

        let combined_probability_distribution = probability_distribution_one
            .combine(probability_distribution_two, |lhs, rhs| lhs - rhs);

        let out = "\
        +-------+-------+\n\
        | value | count |\n\
        +=======+=======+\n\
        +-------+-------+\n\
        ";
        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            out
        );
    }

    #[test]
    fn test_combine_single_outcome_probability_distribution_with_value_type() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(123);
        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);

        let combined_probability_distribution =
            probability_distribution.combine_value_type(321, |lhs, rhs| lhs - rhs);

        let out = "\
        +-------+-------+\n\
        | value | count |\n\
        +=======+=======+\n\
        | -198  | 1     |\n\
        +-------+-------+\n\
        ";
        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            out
        );
    }

    #[test]
    fn test_combine_value_type_with_single_outcome_probability_distribution() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(123);
        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);

        let combined_probability_distribution =
            probability_distribution.value_type_combine(321, |lhs, rhs| lhs - rhs);

        let out = "\
        +-------+-------+\n\
        | value | count |\n\
        +=======+=======+\n\
        | 198   | 1     |\n\
        +-------+-------+\n\
        ";
        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            out
        );
    }

    #[test]
    fn test_combine_single_outcome_probability_distribution_with_single_outcome_probability_distribution(
    ) {
        let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(123);
        let probability_distribution_one =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_one);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(534);
        let probability_distribution_two =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_two);

        let combined_probability_distribution = probability_distribution_one
            .combine(probability_distribution_two, |lhs, rhs| lhs - rhs);

        let out = "\
        +-------+-------+\n\
        | value | count |\n\
        +=======+=======+\n\
        | -411  | 1     |\n\
        +-------+-------+\n\
        ";
        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            out
        );
    }

    #[test]
    fn test_combine_many_outcome_probability_distribution_with_empty_probability_distribution() {
        let probability_distribution_one = ProbabilityDistribution::new_dice(10);
        let probability_distribution_two = ProbabilityDistribution::new_empty_distribution();

        let combined_probability_distribution = probability_distribution_one
            .combine(probability_distribution_two, |lhs, rhs| lhs - rhs);

        let out = "\
        +-------+-------+\n\
        | value | count |\n\
        +=======+=======+\n\
        +-------+-------+\n\
        ";
        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            out
        );
    }

    #[test]
    fn test_combine_many_outcome_probability_distribution_with_value_type() {
        let probability_distribution = ProbabilityDistribution::new_dice(10);
        let combined_probability_distribution =
            probability_distribution.combine_value_type(321, |lhs, rhs| lhs - rhs);

        let out = "\
        +-------+-------+\n\
        | value | count |\n\
        +=======+=======+\n\
        | -320  | 1     |\n\
        +-------+-------+\n\
        | -319  | 1     |\n\
        +-------+-------+\n\
        | -318  | 1     |\n\
        +-------+-------+\n\
        | -317  | 1     |\n\
        +-------+-------+\n\
        | -316  | 1     |\n\
        +-------+-------+\n\
        | -315  | 1     |\n\
        +-------+-------+\n\
        | -314  | 1     |\n\
        +-------+-------+\n\
        | -313  | 1     |\n\
        +-------+-------+\n\
        | -312  | 1     |\n\
        +-------+-------+\n\
        | -311  | 1     |\n\
        +-------+-------+\n\
        ";
        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            out
        );
    }

    #[test]
    fn test_combine_value_type_with_many_outcome_probability_distribution() {
        let probability_distribution = ProbabilityDistribution::new_dice(10);
        let combined_probability_distribution =
            probability_distribution.value_type_combine(100, |lhs, rhs| lhs * rhs);

        let out = "\
        +-------+-------+\n\
        | value | count |\n\
        +=======+=======+\n\
        | 100   | 1     |\n\
        +-------+-------+\n\
        | 200   | 1     |\n\
        +-------+-------+\n\
        | 300   | 1     |\n\
        +-------+-------+\n\
        | 400   | 1     |\n\
        +-------+-------+\n\
        | 500   | 1     |\n\
        +-------+-------+\n\
        | 600   | 1     |\n\
        +-------+-------+\n\
        | 700   | 1     |\n\
        +-------+-------+\n\
        | 800   | 1     |\n\
        +-------+-------+\n\
        | 900   | 1     |\n\
        +-------+-------+\n\
        | 1000  | 1     |\n\
        +-------+-------+\n\
        ";
        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            out
        );
    }

    #[test]
    fn test_combine_many_outcome_probability_distribution_with_many_outcome_probability_distribution(
    ) {
        let probability_distribution_one = ProbabilityDistribution::new_dice(10);
        let probability_distribution_two = ProbabilityDistribution::new_dice(6);

        let combined_probability_distribution = probability_distribution_one
            .combine(probability_distribution_two, |lhs, rhs| lhs + rhs);

        let out = "\
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
        | 7     | 6     |\n\
        +-------+-------+\n\
        | 8     | 6     |\n\
        +-------+-------+\n\
        | 9     | 6     |\n\
        +-------+-------+\n\
        | 10    | 6     |\n\
        +-------+-------+\n\
        | 11    | 6     |\n\
        +-------+-------+\n\
        | 12    | 5     |\n\
        +-------+-------+\n\
        | 13    | 4     |\n\
        +-------+-------+\n\
        | 14    | 3     |\n\
        +-------+-------+\n\
        | 15    | 2     |\n\
        +-------+-------+\n\
        | 16    | 1     |\n\
        +-------+-------+\n\
        ";
        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            out
        );
    }

    #[test]
    fn test_combine_single_outcome_probability_distribution_with_empty_probability_distribution_with_constraints(
    ) {
        let probability_outcome = ProbabilityOutcome::new_with_constraints(
            123,
            vec![Constraint::new_single_valid_value_constraint(12345, 54321)],
        );
        let probability_distribution_one =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);
        let probability_distribution_two = ProbabilityDistribution::new_empty_distribution();

        let combined_probability_distribution = probability_distribution_one
            .combine(probability_distribution_two, |lhs, rhs| lhs - rhs);

        let out = "\
        +-------+-------+\n\
        | value | count |\n\
        +=======+=======+\n\
        +-------+-------+\n\
        ";
        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            out
        );
    }

    #[test]
    fn test_combine_single_outcome_probability_distribution_with_value_type_with_constraints() {
        let probability_outcome = ProbabilityOutcome::new_with_constraints(
            123,
            vec![Constraint::new_single_valid_value_constraint(12345, 54321)],
        );
        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);

        let combined_probability_distribution =
            probability_distribution.combine_value_type(321, |lhs, rhs| lhs - rhs);

        let out = "\
        +-------+-------+-------+\n\
        | value | count | 12345 |\n\
        +=======+=======+=======+\n\
        | -198  | 1     | 54321 |\n\
        +-------+-------+-------+\n\
        ";
        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            out
        );
    }

    #[test]
    fn test_combine_value_type_with_single_outcome_probability_distribution_with_constraints() {
        let probability_outcome = ProbabilityOutcome::new_with_constraints(
            123,
            vec![Constraint::new_single_valid_value_constraint(12345, 54321)],
        );
        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);

        let combined_probability_distribution =
            probability_distribution.value_type_combine(321, |lhs, rhs| lhs - rhs);

        let out = "\
        +-------+-------+-------+\n\
        | value | count | 12345 |\n\
        +=======+=======+=======+\n\
        | 198   | 1     | 54321 |\n\
        +-------+-------+-------+\n\
        ";
        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            out
        );
    }

    #[test]
    fn test_combine_single_outcome_probability_distribution_with_single_outcome_probability_distribution_with_different_constraints(
    ) {
        let probability_outcome_one = ProbabilityOutcome::new_with_constraints(
            123,
            vec![Constraint::new_single_valid_value_constraint(1000, 10)],
        );
        let probability_distribution_one =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_one);
        let probability_outcome_two = ProbabilityOutcome::new_with_constraints(
            234,
            vec![Constraint::new_single_valid_value_constraint(2000, 20)],
        );
        let probability_distribution_two =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_two);

        let combined_probability_distribution = probability_distribution_one
            .combine(probability_distribution_two, |lhs, rhs| lhs - rhs);

        let out = "\
        +-------+-------+------+------+\n\
        | value | count | 1000 | 2000 |\n\
        +=======+=======+======+======+\n\
        | -111  | 1     | 10   | 20   |\n\
        +-------+-------+------+------+\n\
        ";
        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            out
        );
    }

    #[test]
    fn test_combine_single_outcome_probability_distribution_with_single_outcome_probability_distribution_with_same_constraints(
    ) {
        let probability_outcome_one = ProbabilityOutcome::new_with_constraints(
            100,
            vec![Constraint::new_many_item_constraint(
                1000,
                vec![10, 20, 30, 40],
            )],
        );
        let probability_distribution_one =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_one);
        let probability_outcome_two = ProbabilityOutcome::new_with_constraints(
            20,
            vec![Constraint::new_many_item_constraint(
                1000,
                vec![30, 40, 50, 60],
            )],
        );
        let probability_distribution_two =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_two);

        let combined_probability_distribution = probability_distribution_one
            .combine(probability_distribution_two, |lhs, rhs| lhs / rhs);

        let out = "\
        +-------+-------+--------+\n\
        | value | count | 1000   |\n\
        +=======+=======+========+\n\
        | 5     | 1     | 30, 40 |\n\
        +-------+-------+--------+\n\
        ";
        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            out
        );
    }

    #[test]
    fn test_combine_many_outcome_probability_distribution_with_empty_probability_distribution_with_constraints(
    ) {
        let probability_distribution_one =
            ProbabilityDistribution::new_dice(10).add_self_value_constraint(100);
        let probability_distribution_two = ProbabilityDistribution::new_empty_distribution();

        let combined_probability_distribution = probability_distribution_one
            .combine(probability_distribution_two, |lhs, rhs| lhs - rhs);

        let out = "\
        +-------+-------+\n\
        | value | count |\n\
        +=======+=======+\n\
        +-------+-------+\n\
        ";
        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            out
        );
    }

    #[test]
    fn test_combine_many_outcome_probability_distribution_with_value_type_with_constraints() {
        let probability_distribution =
            ProbabilityDistribution::new_dice(10).add_self_value_constraint(100);
        let combined_probability_distribution =
            probability_distribution.combine_value_type(321, |lhs, rhs| lhs - rhs);

        let out = "\
        +-------+-------+-----+\n\
        | value | count | 100 |\n\
        +=======+=======+=====+\n\
        | -320  | 1     | 1   |\n\
        +-------+-------+-----+\n\
        | -319  | 1     | 2   |\n\
        +-------+-------+-----+\n\
        | -318  | 1     | 3   |\n\
        +-------+-------+-----+\n\
        | -317  | 1     | 4   |\n\
        +-------+-------+-----+\n\
        | -316  | 1     | 5   |\n\
        +-------+-------+-----+\n\
        | -315  | 1     | 6   |\n\
        +-------+-------+-----+\n\
        | -314  | 1     | 7   |\n\
        +-------+-------+-----+\n\
        | -313  | 1     | 8   |\n\
        +-------+-------+-----+\n\
        | -312  | 1     | 9   |\n\
        +-------+-------+-----+\n\
        | -311  | 1     | 10  |\n\
        +-------+-------+-----+\n\
        ";
        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            out
        );
    }

    #[test]
    fn test_combine_value_type_with_many_outcome_probability_distribution_with_constraints() {
        let probability_distribution =
            ProbabilityDistribution::new_dice(10).add_self_value_constraint(10);
        let combined_probability_distribution =
            probability_distribution.value_type_combine(100, |lhs, rhs| lhs - rhs);

        let out = "\
        +-------+-------+----+\n\
        | value | count | 10 |\n\
        +=======+=======+====+\n\
        | 90    | 1     | 10 |\n\
        +-------+-------+----+\n\
        | 91    | 1     | 9  |\n\
        +-------+-------+----+\n\
        | 92    | 1     | 8  |\n\
        +-------+-------+----+\n\
        | 93    | 1     | 7  |\n\
        +-------+-------+----+\n\
        | 94    | 1     | 6  |\n\
        +-------+-------+----+\n\
        | 95    | 1     | 5  |\n\
        +-------+-------+----+\n\
        | 96    | 1     | 4  |\n\
        +-------+-------+----+\n\
        | 97    | 1     | 3  |\n\
        +-------+-------+----+\n\
        | 98    | 1     | 2  |\n\
        +-------+-------+----+\n\
        | 99    | 1     | 1  |\n\
        +-------+-------+----+\n\
        ";
        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            out
        );
    }

    #[test]
    fn test_combine_many_outcome_probability_distribution_with_many_outcome_probability_distribution_with_constraints(
    ) {
        let common_constraint_one = Constraint::new_many_item_constraint(30, vec![10, 20, 30]);
        let common_constraint_two = Constraint::new_many_item_constraint(30, vec![20, 30, 40]);

        let probability_distribution_one = ProbabilityDistribution::new_dice(4)
            .add_self_value_constraint(10)
            + common_constraint_one;
        let probability_distribution_two = ProbabilityDistribution::new_dice(4)
            .add_self_value_constraint(20)
            + common_constraint_two;

        let combined_probability_distribution = probability_distribution_one
            .combine(probability_distribution_two, |lhs, rhs| lhs - rhs);

        let out = "\
        +-------+-------+----+----+--------+\n\
        | value | count | 10 | 20 | 30     |\n\
        +=======+=======+====+====+========+\n\
        | -3    | 1     | 1  | 4  | 20, 30 |\n\
        +-------+-------+----+----+--------+\n\
        | -2    | 1     | 1  | 3  | 20, 30 |\n\
        +-------+-------+----+----+--------+\n\
        | -2    | 1     | 2  | 4  | 20, 30 |\n\
        +-------+-------+----+----+--------+\n\
        | -1    | 1     | 1  | 2  | 20, 30 |\n\
        +-------+-------+----+----+--------+\n\
        | -1    | 1     | 2  | 3  | 20, 30 |\n\
        +-------+-------+----+----+--------+\n\
        | -1    | 1     | 3  | 4  | 20, 30 |\n\
        +-------+-------+----+----+--------+\n\
        | 0     | 1     | 1  | 1  | 20, 30 |\n\
        +-------+-------+----+----+--------+\n\
        | 0     | 1     | 2  | 2  | 20, 30 |\n\
        +-------+-------+----+----+--------+\n\
        | 0     | 1     | 3  | 3  | 20, 30 |\n\
        +-------+-------+----+----+--------+\n\
        | 0     | 1     | 4  | 4  | 20, 30 |\n\
        +-------+-------+----+----+--------+\n\
        | 1     | 1     | 2  | 1  | 20, 30 |\n\
        +-------+-------+----+----+--------+\n\
        | 1     | 1     | 3  | 2  | 20, 30 |\n\
        +-------+-------+----+----+--------+\n\
        | 1     | 1     | 4  | 3  | 20, 30 |\n\
        +-------+-------+----+----+--------+\n\
        | 2     | 1     | 3  | 1  | 20, 30 |\n\
        +-------+-------+----+----+--------+\n\
        | 2     | 1     | 4  | 2  | 20, 30 |\n\
        +-------+-------+----+----+--------+\n\
        | 3     | 1     | 4  | 1  | 20, 30 |\n\
        +-------+-------+----+----+--------+\n\
        ";
        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            out
        );
    }

    #[test]
    fn test_combine_many_outcome_probability_distribution_with_many_outcome_probability_distribution_multiple_common_values(
    ) {
        let common_constraint_one = Constraint::new_many_item_constraint(30, vec![10, 20, 30]);
        let common_constraint_two = Constraint::new_many_item_constraint(30, vec![20, 30, 40]);

        let probability_distribution_one = ProbabilityDistribution::new_dice(4)
            .add_self_value_constraint(10)
            + common_constraint_one;
        let probability_distribution_two = ProbabilityDistribution::new_dice(4)
            .add_self_value_constraint(10)
            + common_constraint_two;

        let combined_probability_distribution = probability_distribution_one
            .combine(probability_distribution_two, |lhs, rhs| lhs + rhs);

        let out = "\
        +-------+-------+----+--------+\n\
        | value | count | 10 | 30     |\n\
        +=======+=======+====+========+\n\
        | 2     | 1     | 1  | 20, 30 |\n\
        +-------+-------+----+--------+\n\
        | 4     | 1     | 2  | 20, 30 |\n\
        +-------+-------+----+--------+\n\
        | 6     | 1     | 3  | 20, 30 |\n\
        +-------+-------+----+--------+\n\
        | 8     | 1     | 4  | 20, 30 |\n\
        +-------+-------+----+--------+\n\
        ";
        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            out
        );
    }
}
