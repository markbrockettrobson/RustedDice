use std::ops::Rem;

use crate::{
    probability::{Combine, ProbabilityDistribution},
    ValueType,
};

fn _rem(lhs: ValueType, rhs: ValueType) -> ValueType {
    lhs % rhs
}

impl Rem for ProbabilityDistribution {
    type Output = Self;

    /// Implements the remainder operator for [ProbabilityDistribution].
    /// values are combined using the remainder function.
    /// constraint maps are combined using the ConstraintMap::add function.
    ///
    /// # Arguments
    ///
    /// * `self` - The first [ProbabilityDistribution] operand.
    /// * `other` - The second [ProbabilityDistribution] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityDistribution] after the remainder operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ToTable;
    ///let dice_one = ProbabilityDistribution::new_dice(9);
    ///let dice_two = ProbabilityDistribution::new_dice(3);
    ///
    ///let combined_probability_distribution = dice_one % dice_two;
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
    ///     | 0     | 16    |\n\
    ///     +-------+-------+\n\
    ///     | 1     | 8     |\n\
    ///     +-------+-------+\n\
    ///     | 2     | 3     |\n\
    ///     +-------+-------+\n\
    ///     ");
    /// ```
    fn rem(self, other: Self) -> Self {
        self.combine(other, _rem)
    }
}

impl Rem<ValueType> for ProbabilityDistribution {
    type Output = Self;

    /// Implements the remainder operator for [ProbabilityDistribution] % [ValueType].
    /// values are combined using the remainder function.
    /// constraint map is taken from the [ProbabilityDistribution].
    ///
    /// # Arguments
    ///
    /// * `self` - The [ProbabilityDistribution] operand.
    /// * `other` - The [ValueType] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityDistribution] after the remainder operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ToTable;
    ///let dice = ProbabilityDistribution::new_dice(7);
    ///
    ///assert_eq!(
    ///    (dice % 3)
    ///        .to_table()
    ///        .to_string()
    ///        .replace("\r\n", "\n"),
    ///     "\
    ///     +-------+-------+\n\
    ///     | value | count |\n\
    ///     +=======+=======+\n\
    ///     | 0     | 2     |\n\
    ///     +-------+-------+\n\
    ///     | 1     | 3     |\n\
    ///     +-------+-------+\n\
    ///     | 2     | 2     |\n\
    ///     +-------+-------+\n\
    ///     ");
    /// ```
    fn rem(self, other: ValueType) -> Self {
        self.combine_value_type(other, _rem)
    }
}

impl Rem<ProbabilityDistribution> for ValueType {
    type Output = ProbabilityDistribution;

    /// Implements the remainder operator for [ValueType] % [ProbabilityDistribution].
    /// values are combined using the remainder function.
    /// constraint map is taken from the [ProbabilityDistribution].
    ///
    /// # Arguments
    ///
    /// * `self` - The [ValueType] operand.
    /// * `other` - The [ProbabilityDistribution] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityDistribution] after the remainder operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ToTable;
    ///let dice = ProbabilityDistribution::new_dice(10);
    ///
    ///assert_eq!(
    ///    (25 % dice)
    ///        .to_table()
    ///        .to_string()
    ///        .replace("\r\n", "\n"),
    ///     "\
    ///     +-------+-------+\n\
    ///     | value | count |\n\
    ///     +=======+=======+\n\
    ///     | 0     | 2     |\n\
    ///     +-------+-------+\n\
    ///     | 1     | 5     |\n\
    ///     +-------+-------+\n\
    ///     | 4     | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 5     | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 7     | 1     |\n\
    ///     +-------+-------+\n\
    ///     ");
    /// ```
    fn rem(self, other: ProbabilityDistribution) -> ProbabilityDistribution {
        other.value_type_combine(self, _rem)
    }
}

#[cfg(test)]
mod tests {
    use crate::probability::{ProbabilityDistribution, ProbabilityOutcome, ToTable};

    #[test]
    fn test_rem() {
        let dice_one = ProbabilityDistribution::new_dice(9);
        let dice_two = ProbabilityDistribution::new_dice(3);

        let combined_probability_distribution = dice_one % dice_two;

        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
             +-------+-------+\n\
             | value | count |\n\
             +=======+=======+\n\
             | 0     | 16    |\n\
             +-------+-------+\n\
             | 1     | 8     |\n\
             +-------+-------+\n\
             | 2     | 3     |\n\
             +-------+-------+\n\
             "
        );
    }

    #[test]
    fn test_rem_value_type() {
        let dice = ProbabilityDistribution::new_dice(10);

        assert_eq!(
            (25 % dice).to_table().to_string().replace("\r\n", "\n"),
            "\
             +-------+-------+\n\
             | value | count |\n\
             +=======+=======+\n\
             | 0     | 2     |\n\
             +-------+-------+\n\
             | 1     | 5     |\n\
             +-------+-------+\n\
             | 4     | 1     |\n\
             +-------+-------+\n\
             | 5     | 1     |\n\
             +-------+-------+\n\
             | 7     | 1     |\n\
             +-------+-------+\n\
             "
        );
    }

    #[test]
    fn test_value_type_rem() {
        let dice = ProbabilityDistribution::new_dice(7);

        assert_eq!(
            (dice % 3).to_table().to_string().replace("\r\n", "\n"),
            "\
             +-------+-------+\n\
             | value | count |\n\
             +=======+=======+\n\
             | 0     | 2     |\n\
             +-------+-------+\n\
             | 1     | 3     |\n\
             +-------+-------+\n\
             | 2     | 2     |\n\
             +-------+-------+\n\
             "
        );
    }

    #[test]
    #[should_panic(expected = "attempt to calculate the remainder with a divisor of zero")]
    fn test_rem_by_zero() {
        let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(12);
        let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(0);

        let probability_distribution_one =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_one);
        let probability_distribution_two =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome_two);

        let _ = probability_distribution_one % probability_distribution_two;
    }

    #[test]
    #[should_panic(expected = "attempt to calculate the remainder with a divisor of zero")]
    fn test_rem_value_type_by_zero() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(12);

        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);
        let _ = probability_distribution % 0;
    }

    #[test]
    #[should_panic(expected = "attempt to calculate the remainder with a divisor of zero")]
    fn test_value_type_rem_by_zero() {
        let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(0);
        let probability_distribution =
            ProbabilityDistribution::new_from_single_probability_outcome(probability_outcome);
        let _ = 3 % probability_distribution;
    }
}
