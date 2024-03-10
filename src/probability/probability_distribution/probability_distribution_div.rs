use std::ops::Div;

use crate::{
    probability::{Combine, ProbabilityDistribution},
    ValueType,
};

fn _div(lhs: ValueType, rhs: ValueType) -> ValueType {
    lhs / rhs
}

impl Div for ProbabilityDistribution {
    type Output = Self;

    /// Implements the divide operator for [ProbabilityDistribution].
    /// values are combined using the divide function.
    /// constraint maps are combined using the ConstraintMap::add function.
    ///
    /// # Arguments
    ///
    /// * `self` - The first [ProbabilityDistribution] operand.
    /// * `other` - The second [ProbabilityDistribution] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityDistribution] after the divide operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ToTable;
    ///let dice_one = ProbabilityDistribution::new_dice(9);
    ///let dice_two = ProbabilityDistribution::new_dice(3);
    ///
    ///let combined_probability_distribution = dice_one / dice_two;
    ///
    ///assert_eq!(
    ///    combined_probability_distribution
    ///        .to_table()
    ///        .to_string()
    ///        .replace("\r\n", "\n"),
    ///     "\
    ///     +-------+-------+\n\
    ///     | value | count |\n\
    ///     +=======+=======+\n\
    ///     | 0     | 3     |\n\
    ///     +-------+-------+\n\
    ///     | 1     | 6     |\n\
    ///     +-------+-------+\n\
    ///     | 2     | 6     |\n\
    ///     +-------+-------+\n\
    ///     | 3     | 4     |\n\
    ///     +-------+-------+\n\
    ///     | 4     | 3     |\n\
    ///     +-------+-------+\n\
    ///     | 5     | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 6     | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 7     | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 8     | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 9     | 1     |\n\
    ///     +-------+-------+\n\
    ///     ");
    /// ```
    fn div(self, other: Self) -> Self {
        self.combine(other, _div)
    }
}

impl Div<ValueType> for ProbabilityDistribution {
    type Output = Self;

    /// Implements the divide operator for [ProbabilityDistribution] / [ValueType].
    /// values are combined using the divide function.
    /// constraint map is taken from the [ProbabilityDistribution].
    ///
    /// # Arguments
    ///
    /// * `self` - The [ProbabilityDistribution] operand.
    /// * `other` - The [ValueType] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityDistribution] after the divide operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ToTable;
    ///let dice = ProbabilityDistribution::new_dice(6);
    ///
    ///assert_eq!(
    ///    (dice / 2)
    ///        .to_table()
    ///        .to_string()
    ///        .replace("\r\n", "\n"),
    ///     "\
    ///     +-------+-------+\n\
    ///     | value | count |\n\
    ///     +=======+=======+\n\
    ///     | 0     | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 1     | 2     |\n\
    ///     +-------+-------+\n\
    ///     | 2     | 2     |\n\
    ///     +-------+-------+\n\
    ///     | 3     | 1     |\n\
    ///     +-------+-------+\n\
    ///     ");
    /// ```
    fn div(self, other: ValueType) -> Self {
        self.combine_value_type(other, _div)
    }
}

impl Div<ProbabilityDistribution> for ValueType {
    type Output = ProbabilityDistribution;

    /// Implements the divide operator for [ValueType] / [ProbabilityDistribution].
    /// values are combined using the divide function.
    /// constraint map is taken from the [ProbabilityDistribution].
    ///
    /// # Arguments
    ///
    /// * `self` - The [ValueType] operand.
    /// * `other` - The [ProbabilityDistribution] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityDistribution] after the divide operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ToTable;
    ///let dice = ProbabilityDistribution::new_dice(4);
    ///
    ///assert_eq!(
    ///    (24 / dice)
    ///        .to_table()
    ///        .to_string()
    ///        .replace("\r\n", "\n"),
    ///     "\
    ///     +-------+-------+\n\
    ///     | value | count |\n\
    ///     +=======+=======+\n\
    ///     | 6     | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 8     | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 12    | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 24    | 1     |\n\
    ///     +-------+-------+\n\
    ///     ");
    /// ```
    fn div(self, other: ProbabilityDistribution) -> ProbabilityDistribution {
        other.value_type_combine(self, _div)
    }
}

#[cfg(test)]
mod tests {
    use crate::probability::{ProbabilityDistribution, ProbabilityOutcome, ToTable};

    #[test]
    fn test_div() {
        let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(12);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(3);

        let probability_distribution_one =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_one);
        let probability_distribution_two =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_two);

        let combined_probability_distribution =
            probability_distribution_one / probability_distribution_two;

        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
            +-------+-------+\n\
            | value | count |\n\
            +=======+=======+\n\
            | 4     | 1     |\n\
            +-------+-------+\n\
            "
        );
    }

    #[test]
    fn test_div_value_type() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(12);

        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);

        let combined_probability_distribution = probability_distribution / 3;

        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
            +-------+-------+\n\
            | value | count |\n\
            +=======+=======+\n\
            | 4     | 1     |\n\
            +-------+-------+\n\
            "
        );
    }

    #[test]
    fn test_value_type_div() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(3);

        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);

        let combined_probability_distribution = 30 / probability_distribution;

        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
            +-------+-------+\n\
            | value | count |\n\
            +=======+=======+\n\
            | 10    | 1     |\n\
            +-------+-------+\n\
            "
        );
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_div_by_zero() {
        let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(12);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(0);

        let probability_distribution_one =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_one);
        let probability_distribution_two =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_two);

        let _ = probability_distribution_one / probability_distribution_two;
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_div_value_type_by_zero() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(12);

        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);
        let _ = probability_distribution / 0;
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_value_type_div_by_zero() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(0);
        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);
        let _ = 3 / probability_distribution;
    }
}
