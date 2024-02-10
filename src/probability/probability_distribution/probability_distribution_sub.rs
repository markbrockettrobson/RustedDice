use std::ops::Sub;

use crate::{
    probability::{Combine, ProbabilityDistribution},
    ValueType,
};

fn _sub(lhs: ValueType, rhs: ValueType) -> ValueType {
    lhs - rhs
}

impl Sub for ProbabilityDistribution {
    type Output = Self;

    /// Implements the subtraction operator for [ProbabilityDistribution].
    /// values are combined using the subtraction function.
    /// constraint maps are combined using the ConstraintMap::add function.
    ///
    /// # Arguments
    ///
    /// * `self` - The first [ProbabilityDistribution] operand.
    /// * `other` - The second [ProbabilityDistribution] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityDistribution] after the subtraction operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ToTable;
    ///let dice_one = ProbabilityDistribution::new_dice(3);
    ///let dice_two = ProbabilityDistribution::new_dice(3);
    ///
    ///let combined_probability_distribution = dice_one - dice_two;
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
    ///     | -2    | 1     |\n\
    ///     +-------+-------+\n\
    ///     | -1    | 2     |\n\
    ///     +-------+-------+\n\
    ///     | 0     | 3     |\n\
    ///     +-------+-------+\n\
    ///     | 1     | 2     |\n\
    ///     +-------+-------+\n\
    ///     | 2     | 1     |\n\
    ///     +-------+-------+\n\
    ///     ");
    /// ```
    fn sub(self, other: Self) -> Self {
        self.combine(other, _sub)
    }
}

impl Sub<ValueType> for ProbabilityDistribution {
    type Output = Self;

    /// Implements the subtraction operator for [ProbabilityDistribution] + [ValueType].
    /// values are combined using the add function.
    /// constraint map is taken from the [ProbabilityDistribution].
    ///
    /// # Arguments
    ///
    /// * `self` - The [ProbabilityDistribution] operand.
    /// * `other` - The [ValueType] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityDistribution] after the subtraction operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ToTable;
    ///let dice = ProbabilityDistribution::new_dice(3);
    ///
    ///let combined_probability_distribution = dice - 10;
    ///
    ///assert_eq!(
    ///    combined_probability_distribution
    ///        .to_table()
    ///        .to_string()
    ///        .replace("\r\n", "\n"),
    ///    "\
    ///    +-------+-------+\n\
    ///    | value | count |\n\
    ///    +=======+=======+\n\
    ///    | -9    | 1     |\n\
    ///    +-------+-------+\n\
    ///    | -8    | 1     |\n\
    ///    +-------+-------+\n\
    ///    | -7    | 1     |\n\
    ///    +-------+-------+\n\
    ///    ");
    /// ```
    fn sub(self, other: ValueType) -> Self {
        self.combine_value_type(other, _sub)
    }
}

impl Sub<ProbabilityDistribution> for ValueType {
    type Output = ProbabilityDistribution;

    /// Implements the subtraction operator for [ValueType] + [ProbabilityDistribution].
    /// values are combined using the add function.
    /// constraint map is taken from the [ProbabilityDistribution].
    ///
    /// # Arguments
    ///
    /// * `self` - The [ValueType] operand.
    /// * `other` - The [ProbabilityDistribution] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityDistribution] after the subtraction operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ToTable;
    ///let dice = ProbabilityDistribution::new_dice(3);
    ///
    ///let combined_probability_distribution = 10 - dice;
    ///
    ///assert_eq!(
    ///    combined_probability_distribution
    ///        .to_table()
    ///        .to_string()
    ///        .replace("\r\n", "\n"),
    ///    "\
    ///    +-------+-------+\n\
    ///    | value | count |\n\
    ///    +=======+=======+\n\
    ///    | 7     | 1     |\n\
    ///    +-------+-------+\n\
    ///    | 8     | 1     |\n\
    ///    +-------+-------+\n\
    ///    | 9     | 1     |\n\
    ///    +-------+-------+\n\
    ///    ");
    /// ```
    fn sub(self, other: ProbabilityDistribution) -> ProbabilityDistribution {
        other.value_type_combine(self, _sub)
    }
}

#[cfg(test)]
mod tests {
    use crate::probability::{ProbabilityDistribution, ProbabilityOutcome, ToTable};
    use crate::ValueType;

    #[test]
    fn test_sub() {
        let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(9999);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(8765);

        let probability_distribution_one =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_one);
        let probability_distribution_two =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_two);

        let combined_probability_distribution =
            probability_distribution_one - probability_distribution_two;

        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
            +-------+-------+\n\
            | value | count |\n\
            +=======+=======+\n\
            | 1234  | 1     |\n\
            +-------+-------+\n\
            "
        );
    }

    #[test]
    fn test_sub_value_type() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(9999);
        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);

        let combined_probability_distribution = probability_distribution - 1234;

        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
            +-------+-------+\n\
            | value | count |\n\
            +=======+=======+\n\
            | 8765  | 1     |\n\
            +-------+-------+\n\
            "
        );
    }

    #[test]
    fn test_value_type_sub() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(1234);

        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);

        let combined_probability_distribution = 9999 - probability_distribution;

        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
            +-------+-------+\n\
            | value | count |\n\
            +=======+=======+\n\
            | 8765  | 1     |\n\
            +-------+-------+\n\
            "
        );
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_sub_overflow() {
        let probability_outcome_one =
            ProbabilityOutcome::new_with_empty_constraint_map(ValueType::MAX);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(-1);

        let probability_distribution_one =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_one);
        let probability_distribution_two =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_two);

        let _ = probability_distribution_one - probability_distribution_two;
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_sub_underflow() {
        let probability_outcome_one =
            ProbabilityOutcome::new_with_empty_constraint_map(ValueType::MIN);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(1);

        let probability_distribution_one =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_one);
        let probability_distribution_two =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_two);

        let _ = probability_distribution_one - probability_distribution_two;
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_sub_value_type_overflow() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(ValueType::MAX);

        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);
        let _ = probability_distribution - -1;
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_sub_value_type_underflow() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(ValueType::MIN);

        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);
        let _ = probability_distribution - 1;
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_value_type_sub_overflow() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(-1);

        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);
        let _ = ValueType::MAX - probability_distribution;
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_value_type_sub_underflow() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(1);

        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);
        let _ = ValueType::MIN - probability_distribution;
    }
}
