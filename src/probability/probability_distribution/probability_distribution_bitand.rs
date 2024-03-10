use std::ops::BitAnd;

use crate::{
    probability::{Combine, ProbabilityDistribution},
    ValueType,
};

fn _bitand(lhs: ValueType, rhs: ValueType) -> ValueType {
    lhs & rhs
}

impl BitAnd for ProbabilityDistribution {
    type Output = Self;

    /// Implements the bitwise and operator for [ProbabilityDistribution].
    /// values are combined using the bitwise and function.
    /// constraint maps are combined using the ConstraintMap::add function.
    ///
    /// # Arguments
    ///
    /// * `self` - The first [ProbabilityDistribution] operand.
    /// * `other` - The second [ProbabilityDistribution] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityDistribution] after the bitwise and operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ToTable;
    ///let dice_one = ProbabilityDistribution::new_dice(3);
    ///let dice_two = ProbabilityDistribution::new_dice(3);
    ///
    ///let combined_probability_distribution = dice_one & dice_two;
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
    ///     | 0     | 2     |\n\
    ///     +-------+-------+\n\
    ///     | 1     | 3     |\n\
    ///     +-------+-------+\n\
    ///     | 2     | 3     |\n\
    ///     +-------+-------+\n\
    ///     | 3     | 1     |\n\
    ///     +-------+-------+\n\
    ///     ");
    /// ```
    fn bitand(self, other: Self) -> Self {
        self.combine(other, _bitand)
    }
}

impl BitAnd<ValueType> for ProbabilityDistribution {
    type Output = Self;

    /// Implements the bitwise and operator for [ProbabilityDistribution] & [ValueType].
    /// values are combined using the bitwise and function.
    /// constraint map is taken from the [ProbabilityDistribution].
    ///
    /// # Arguments
    ///
    /// * `self` - The [ProbabilityDistribution] operand.
    /// * `other` - The [ValueType] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityDistribution] after the bitwise and operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ToTable;
    ///let dice = ProbabilityDistribution::new_dice(6);
    ///
    ///let combined_probability_distribution = dice & 12;
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
    ///     | 4     | 3     |\n\
    ///     +-------+-------+\n\
    ///     ");
    /// ```
    fn bitand(self, other: ValueType) -> Self {
        self.combine_value_type(other, _bitand)
    }
}

impl BitAnd<ProbabilityDistribution> for ValueType {
    type Output = ProbabilityDistribution;

    /// Implements the bitwise and operator for [ValueType] & [ProbabilityDistribution].
    /// values are combined using the bitwise and function.
    /// constraint map is taken from the [ProbabilityDistribution].
    ///
    /// # Arguments
    ///
    /// * `self` - The [ValueType] operand.
    /// * `other` - The [ProbabilityDistribution] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityDistribution] after the bitwise and operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ToTable;
    ///let dice = ProbabilityDistribution::new_dice(4);
    ///
    ///let combined_probability_distribution = 42 & dice;
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
    ///     | 0     | 2     |\n\
    ///     +-------+-------+\n\
    ///     | 2     | 2     |\n\
    ///     +-------+-------+\n\
    ///     ");
    /// ```
    fn bitand(self, other: ProbabilityDistribution) -> ProbabilityDistribution {
        other.value_type_combine(self, _bitand)
    }
}

#[cfg(test)]
mod tests {
    use crate::probability::ProbabilityDistribution;
    use crate::probability::ToTable;

    #[test]
    fn test_bitand() {
        let dice_one = ProbabilityDistribution::new_dice(3);
        let dice_two = ProbabilityDistribution::new_dice(3);

        let combined_probability_distribution = dice_one & dice_two;

        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
             +-------+-------+\n\
             | value | count |\n\
             +=======+=======+\n\
             | 0     | 2     |\n\
             +-------+-------+\n\
             | 1     | 3     |\n\
             +-------+-------+\n\
             | 2     | 3     |\n\
             +-------+-------+\n\
             | 3     | 1     |\n\
             +-------+-------+\n\
             "
        );
    }

    #[test]
    fn test_bitand_value_type() {
        let dice = ProbabilityDistribution::new_dice(6);

        let combined_probability_distribution = dice & 12;

        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
             +-------+-------+\n\
             | value | count |\n\
             +=======+=======+\n\
             | 0     | 3     |\n\
             +-------+-------+\n\
             | 4     | 3     |\n\
             +-------+-------+\n\
             "
        );
    }

    #[test]
    fn test_value_type_bitand() {
        let dice = ProbabilityDistribution::new_dice(4);

        let combined_probability_distribution = 42 & dice;

        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
             +-------+-------+\n\
             | value | count |\n\
             +=======+=======+\n\
             | 0     | 2     |\n\
             +-------+-------+\n\
             | 2     | 2     |\n\
             +-------+-------+\n\
             "
        );
    }
}
