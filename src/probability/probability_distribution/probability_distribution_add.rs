use std::ops::Add;

use crate::{
    probability::{Combine, ProbabilityDistribution},
    ValueType,
};

fn _add(lhs: ValueType, rhs: ValueType) -> ValueType {
    lhs + rhs
}

impl Add for ProbabilityDistribution {
    type Output = Self;

    /// Implements the addition operator for [ProbabilityDistribution].
    /// values are combined using the add function.
    /// constraint maps are combined using the ConstraintMap::add function.
    ///
    /// # Arguments
    ///
    /// * `self` - The first [ProbabilityDistribution] operand.
    /// * `other` - The second [ProbabilityDistribution] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityDistribution] after the addition operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ToTable;
    ///let dice_one = ProbabilityDistribution::new_dice(3);
    ///let dice_two = ProbabilityDistribution::new_dice(3);
    ///
    ///let combined_probability_distribution = dice_one + dice_two;
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
    ///     | 2     | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 3     | 2     |\n\
    ///     +-------+-------+\n\
    ///     | 4     | 3     |\n\
    ///     +-------+-------+\n\
    ///     | 5     | 2     |\n\
    ///     +-------+-------+\n\
    ///     | 6     | 1     |\n\
    ///     +-------+-------+\n\
    ///     ");
    /// ```
    fn add(self, other: Self) -> Self {
        self.combine(other, _add)
    }
}

impl Add<ValueType> for ProbabilityDistribution {
    type Output = Self;

    /// Implements the addition operator for [ProbabilityDistribution] + [ValueType].
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
    /// The resulting [ProbabilityDistribution] after the addition operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ToTable;
    ///let dice = ProbabilityDistribution::new_dice(3);
    ///
    ///let combined_probability_distribution = dice + 10;
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
    ///    | 11    | 1     |\n\
    ///    +-------+-------+\n\
    ///    | 12    | 1     |\n\
    ///    +-------+-------+\n\
    ///    | 13    | 1     |\n\
    ///    +-------+-------+\n\
    ///    ");
    /// ```
    fn add(self, other: ValueType) -> Self {
        self.combine_value_type(other, _add)
    }
}

impl Add<ProbabilityDistribution> for ValueType {
    type Output = ProbabilityDistribution;

    /// Implements the addition operator for [ValueType] + [ProbabilityDistribution].
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
    /// The resulting [ProbabilityDistribution] after the addition operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ToTable;
    ///let dice = ProbabilityDistribution::new_dice(3);
    ///
    ///let combined_probability_distribution = 10 + dice;
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
    ///    | 11    | 1     |\n\
    ///    +-------+-------+\n\
    ///    | 12    | 1     |\n\
    ///    +-------+-------+\n\
    ///    | 13    | 1     |\n\
    ///    +-------+-------+\n\
    ///    ");
    /// ```
    fn add(self, other: ProbabilityDistribution) -> ProbabilityDistribution {
        other.value_type_combine(self, _add)
    }
}

#[cfg(test)]
mod tests {
    use crate::probability::{ProbabilityDistribution, ProbabilityOutcome, ToTable};
    use crate::ValueType;

    #[test]
    fn test_add() {
        let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(1234);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(8765);

        let probability_distribution_one =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_one);
        let probability_distribution_two =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_two);

        let combined_probability_distribution =
            probability_distribution_one + probability_distribution_two;

        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
            +-------+-------+\n\
            | value | count |\n\
            +=======+=======+\n\
            | 9999  | 1     |\n\
            +-------+-------+\n\
            "
        );
    }

    #[test]
    fn test_add_value_type() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(1234);
        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);

        let combined_probability_distribution = probability_distribution + 8765;

        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
            +-------+-------+\n\
            | value | count |\n\
            +=======+=======+\n\
            | 9999  | 1     |\n\
            +-------+-------+\n\
            "
        );
    }

    #[test]
    fn test_value_type_add() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(1234);

        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);

        let combined_probability_distribution = 8765 + probability_distribution;

        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
            +-------+-------+\n\
            | value | count |\n\
            +=======+=======+\n\
            | 9999  | 1     |\n\
            +-------+-------+\n\
            "
        );
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_add_overflow() {
        let probability_outcome_one =
            ProbabilityOutcome::new_with_empty_constraint_map(ValueType::MAX);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(1);

        let probability_distribution_one =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_one);
        let probability_distribution_two =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_two);

        let _ = probability_distribution_one + probability_distribution_two;
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_add_underflow() {
        let probability_outcome_one =
            ProbabilityOutcome::new_with_empty_constraint_map(ValueType::MIN);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(-11);

        let probability_distribution_one =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_one);
        let probability_distribution_two =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_two);

        let _ = probability_distribution_one + probability_distribution_two;
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_add_value_type_overflow() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(ValueType::MAX);

        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);
        let _ = probability_distribution + 1;
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_add_value_type_underflow() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(ValueType::MIN);

        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);
        let _ = probability_distribution + -1;
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_value_type_add_overflow() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(ValueType::MAX);

        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);
        let _ = 1 + probability_distribution;
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_value_type_add_underflow() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(ValueType::MIN);

        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);
        let _ = -1 + probability_distribution;
    }
}
