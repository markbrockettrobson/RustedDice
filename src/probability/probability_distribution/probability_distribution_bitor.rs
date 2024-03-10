use std::ops::BitOr;

use crate::{
    probability::{Combine, ProbabilityDistribution},
    ValueType,
};

fn _bitor(lhs: ValueType, rhs: ValueType) -> ValueType {
    lhs | rhs
}

impl BitOr for ProbabilityDistribution {
    type Output = Self;

    /// Implements the bitwise or operator for [ProbabilityDistribution].
    /// values are combined using the bitwise or function.
    /// constraint maps are combined using the ConstraintMap::add function.
    ///
    /// # Arguments
    ///
    /// * `self` - The first [ProbabilityDistribution] operand.
    /// * `other` - The second [ProbabilityDistribution] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityDistribution] after the bitwise or operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ToTable;
    ///let dice_one = ProbabilityDistribution::new_dice(3);
    ///let dice_two = ProbabilityDistribution::new_dice(3);
    ///
    ///let combined_probability_distribution = dice_one | dice_two;
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
    ///     | 2     | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 3     | 7     |\n\
    ///     +-------+-------+\n\
    ///     ");
    /// ```
    fn bitor(self, other: Self) -> Self {
        self.combine(other, _bitor)
    }
}

impl BitOr<ValueType> for ProbabilityDistribution {
    type Output = Self;

    /// Implements the bitwise or operator for [ProbabilityDistribution] | [ValueType].
    /// values are combined using the bitwise or function.
    /// constraint map is taken from the [ProbabilityDistribution].
    ///
    /// # Arguments
    ///
    /// * `self` - The [ProbabilityDistribution] operand.
    /// * `other` - The [ValueType] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityDistribution] after the bitwise or operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ToTable;
    ///let dice = ProbabilityDistribution::new_dice(6);
    ///
    ///let combined_probability_distribution = dice | 12;
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
    ///     | 12    | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 13    | 2     |\n\
    ///     +-------+-------+\n\
    ///     | 14    | 2     |\n\
    ///     +-------+-------+\n\
    ///     | 15    | 1     |\n\
    ///     +-------+-------+\n\
    ///     ");
    /// ```
    fn bitor(self, other: ValueType) -> Self {
        self.combine_value_type(other, _bitor)
    }
}

impl BitOr<ProbabilityDistribution> for ValueType {
    type Output = ProbabilityDistribution;

    /// Implements the bitwise or operator for [ValueType] | [ProbabilityDistribution].
    /// values are combined using the bitwise or function.
    /// constraint map is taken from the [ProbabilityDistribution].
    ///
    /// # Arguments
    ///
    /// * `self` - The [ValueType] operand.
    /// * `other` - The [ProbabilityDistribution] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityDistribution] after the bitwise or operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ToTable;
    ///let dice = ProbabilityDistribution::new_dice(4);
    ///
    ///let combined_probability_distribution = 42 | dice;
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
    ///     | 42    | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 43    | 2     |\n\
    ///     +-------+-------+\n\
    ///     | 46    | 1     |\n\
    ///     +-------+-------+\n\
    ///     ");
    /// ```
    fn bitor(self, other: ProbabilityDistribution) -> ProbabilityDistribution {
        other.value_type_combine(self, _bitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::probability::ProbabilityDistribution;
    use crate::probability::ToTable;

    #[test]
    fn test_bitor() {
        let dice_one = ProbabilityDistribution::new_dice(3);
        let dice_two = ProbabilityDistribution::new_dice(3);

        let combined_probability_distribution = dice_one | dice_two;

        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
             +-------+-------+\n\
             | value | count |\n\
             +=======+=======+\n\
             | 1     | 1     |\n\
             +-------+-------+\n\
             | 2     | 1     |\n\
             +-------+-------+\n\
             | 3     | 7     |\n\
             +-------+-------+\n\
             "
        );
    }

    #[test]
    fn test_bitor_value_type() {
        let dice = ProbabilityDistribution::new_dice(6);

        let combined_probability_distribution = dice | 12;

        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
             +-------+-------+\n\
             | value | count |\n\
             +=======+=======+\n\
             | 12    | 1     |\n\
             +-------+-------+\n\
             | 13    | 2     |\n\
             +-------+-------+\n\
             | 14    | 2     |\n\
             +-------+-------+\n\
             | 15    | 1     |\n\
             +-------+-------+\n\
             "
        );
    }

    #[test]
    fn test_value_type_bitor() {
        let dice = ProbabilityDistribution::new_dice(4);

        let combined_probability_distribution = 42 | dice;

        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
            +-------+-------+\n\
            | value | count |\n\
            +=======+=======+\n\
            | 42    | 1     |\n\
            +-------+-------+\n\
            | 43    | 2     |\n\
            +-------+-------+\n\
            | 46    | 1     |\n\
            +-------+-------+\n\
            "
        );
    }
}
