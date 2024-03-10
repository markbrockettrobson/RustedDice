use std::ops::Mul;

use crate::{
    probability::{Combine, ProbabilityDistribution},
    ValueType,
};

fn _mul(lhs: ValueType, rhs: ValueType) -> ValueType {
    lhs * rhs
}

impl Mul for ProbabilityDistribution {
    type Output = Self;

    /// Implements the multiply operator for [ProbabilityDistribution].
    /// values are combined using the multiply function.
    /// constraint maps are combined using the ConstraintMap::add function.
    ///
    /// # Arguments
    ///
    /// * `self` - The first [ProbabilityDistribution] operand.
    /// * `other` - The second [ProbabilityDistribution] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityDistribution] after the multiply operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ToTable;
    ///let dice_one = ProbabilityDistribution::new_dice(3);
    ///let dice_two = ProbabilityDistribution::new_dice(3);
    ///
    ///let combined_probability_distribution = dice_one * dice_two;
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
    ///     | 1     | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 2     | 2     |\n\
    ///     +-------+-------+\n\
    ///     | 3     | 2     |\n\
    ///     +-------+-------+\n\
    ///     | 4     | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 6     | 2     |\n\
    ///     +-------+-------+\n\
    ///     | 9     | 1     |\n\
    ///     +-------+-------+\n\
    ///     ");
    /// ```
    fn mul(self, other: Self) -> Self {
        self.combine(other, _mul)
    }
}

impl Mul<ValueType> for ProbabilityDistribution {
    type Output = Self;

    /// Implements the multiply operator for [ProbabilityDistribution] * [ValueType].
    /// values are combined using the multiply function.
    /// constraint map is taken from the [ProbabilityDistribution].
    ///
    /// # Arguments
    ///
    /// * `self` - The [ProbabilityDistribution] operand.
    /// * `other` - The [ValueType] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityDistribution] after the multiply operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ToTable;
    ///let dice = ProbabilityDistribution::new_dice(3);
    ///
    ///assert_eq!(
    ///    (dice * 30)
    ///        .to_table()
    ///        .to_string()
    ///        .replace("\r\n", "\n"),
    ///     "\
    ///     +-------+-------+\n\
    ///     | value | count |\n\
    ///     +=======+=======+\n\
    ///     | 30    | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 60    | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 90    | 1     |\n\
    ///     +-------+-------+\n\
    ///     ");
    /// ```
    fn mul(self, other: ValueType) -> Self {
        self.combine_value_type(other, _mul)
    }
}

impl Mul<ProbabilityDistribution> for ValueType {
    type Output = ProbabilityDistribution;

    /// Implements the multiply operator for [ValueType] * [ProbabilityDistribution].
    /// values are combined using the multiply function.
    /// constraint map is taken from the [ProbabilityDistribution].
    ///
    /// # Arguments
    ///
    /// * `self` - The [ValueType] operand.
    /// * `other` - The [ProbabilityDistribution] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityDistribution] after the multiply operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ToTable;
    ///let dice = ProbabilityDistribution::new_dice(4);
    ///
    ///assert_eq!(
    ///    (-1 * dice)
    ///        .to_table()
    ///        .to_string()
    ///        .replace("\r\n", "\n"),
    ///     "\
    ///     +-------+-------+\n\
    ///     | value | count |\n\
    ///     +=======+=======+\n\
    ///     | -4    | 1     |\n\
    ///     +-------+-------+\n\
    ///     | -3    | 1     |\n\
    ///     +-------+-------+\n\
    ///     | -2    | 1     |\n\
    ///     +-------+-------+\n\
    ///     | -1    | 1     |\n\
    ///     +-------+-------+\n\
    ///     ");
    /// ```
    fn mul(self, other: ProbabilityDistribution) -> ProbabilityDistribution {
        other.value_type_combine(self, _mul)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        probability::{ProbabilityDistribution, ProbabilityOutcome, ToTable},
        ValueType,
    };

    #[test]
    fn test_mul() {
        let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(11);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(12);

        let probability_distribution_one =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_one);
        let probability_distribution_two =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_two);

        let combined_probability_distribution =
            probability_distribution_one * probability_distribution_two;

        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
            +-------+-------+\n\
            | value | count |\n\
            +=======+=======+\n\
            | 132   | 1     |\n\
            +-------+-------+\n\
            "
        );
    }

    #[test]
    fn test_mul_value_type() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(11);

        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);

        let combined_probability_distribution = probability_distribution * 44;

        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
            +-------+-------+\n\
            | value | count |\n\
            +=======+=======+\n\
            | 484   | 1     |\n\
            +-------+-------+\n\
            "
        );
    }

    #[test]
    fn test_value_type_mul() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(11);

        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);

        let combined_probability_distribution = 44 * probability_distribution;

        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
            +-------+-------+\n\
            | value | count |\n\
            +=======+=======+\n\
            | 484   | 1     |\n\
            +-------+-------+\n\
            "
        );
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_mul_overflow() {
        let probability_outcome_one =
            ProbabilityOutcome::new_with_empty_constraint_map(ValueType::MAX);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(2);

        let probability_distribution_one =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_one);
        let probability_distribution_two =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_two);

        let _ = probability_distribution_one * probability_distribution_two;
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_mul_value_type_overflow() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(ValueType::MAX);

        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);
        let _ = probability_distribution * 2;
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_value_type_mul_overflow() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(2);

        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);

        let _ = ValueType::MAX * probability_distribution;
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_mul_underflow() {
        let probability_outcome_one =
            ProbabilityOutcome::new_with_empty_constraint_map(ValueType::MAX);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(-2);

        let probability_distribution_one =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_one);
        let probability_distribution_two =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_two);

        let _ = probability_distribution_one * probability_distribution_two;
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_mul_value_type_underflow() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(ValueType::MAX);

        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);
        let _ = probability_distribution * -2;
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_value_type_mul_underflow() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(-2);
        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);
        let _ = ValueType::MAX * probability_distribution;
    }
}
