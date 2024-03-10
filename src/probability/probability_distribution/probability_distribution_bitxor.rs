use std::ops::BitXor;

use crate::{
    probability::{Combine, ProbabilityDistribution},
    ValueType,
};

fn _bitxor(lhs: ValueType, rhs: ValueType) -> ValueType {
    lhs ^ rhs
}

impl BitXor for ProbabilityDistribution {
    type Output = Self;

    /// Implements the bitwise xor operator for [ProbabilityDistribution].
    /// values are combined using the bitwise xor function.
    /// constraint maps are combined using the ConstraintMap::add function.
    ///
    /// # Arguments
    ///
    /// * `self` - The first [ProbabilityDistribution] operand.
    /// * `other` - The second [ProbabilityDistribution] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityDistribution] after the bitwise xor operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ToTable;
    ///let dice_one = ProbabilityDistribution::new_dice(3);
    ///let dice_two = ProbabilityDistribution::new_dice(3);
    ///
    ///let combined_probability_distribution = dice_one ^ dice_two;
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
    ///     | 1     | 2     |\n\
    ///     +-------+-------+\n\
    ///     | 2     | 2     |\n\
    ///     +-------+-------+\n\
    ///     | 3     | 2     |\n\
    ///     +-------+-------+\n\
    ///     ");
    /// ```
    fn bitxor(self, other: Self) -> Self {
        self.combine(other, _bitxor)
    }
}

impl BitXor<ValueType> for ProbabilityDistribution {
    type Output = Self;

    /// Implements the bitwise xor operator for [ProbabilityDistribution] ^ [ValueType].
    /// values are combined using the bitwise xor function.
    /// constraint map is taken from the [ProbabilityDistribution].
    ///
    /// # Arguments
    ///
    /// * `self` - The [ProbabilityDistribution] operand.
    /// * `other` - The [ValueType] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityDistribution] after the bitwise xor operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ToTable;
    ///let dice = ProbabilityDistribution::new_dice(6);
    ///
    ///let combined_probability_distribution = dice ^ 12;
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
    ///     | 8     | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 9     | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 10    | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 13    | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 14    | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 15    | 1     |\n\
    ///     +-------+-------+\n\
    ///     ");
    /// ```
    fn bitxor(self, other: ValueType) -> Self {
        self.combine_value_type(other, _bitxor)
    }
}

impl BitXor<ProbabilityDistribution> for ValueType {
    type Output = ProbabilityDistribution;

    /// Implements the bitwise xor operator for [ValueType] ^ [ProbabilityDistribution].
    /// values are combined using the bitwise xor function.
    /// constraint map is taken from the [ProbabilityDistribution].
    ///
    /// # Arguments
    ///
    /// * `self` - The [ValueType] operand.
    /// * `other` - The [ProbabilityDistribution] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityDistribution] after the bitwise xor operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ToTable;
    ///let dice = ProbabilityDistribution::new_dice(4);
    ///
    ///let combined_probability_distribution = 42 ^ dice;
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
    ///     | 40    | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 41    | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 43    | 1     |\n\
    ///     +-------+-------+\n\
    ///     | 46    | 1     |\n\
    ///     +-------+-------+\n\
    ///     ");
    /// ```
    fn bitxor(self, other: ProbabilityDistribution) -> ProbabilityDistribution {
        other.value_type_combine(self, _bitxor)
    }
}

#[cfg(test)]
mod tests {
    use crate::probability::ProbabilityDistribution;
    use crate::probability::ToTable;

    #[test]
    fn test_bitxor() {
        let dice_one = ProbabilityDistribution::new_dice(3);
        let dice_two = ProbabilityDistribution::new_dice(3);

        let combined_probability_distribution = dice_one ^ dice_two;

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
             | 1     | 2     |\n\
             +-------+-------+\n\
             | 2     | 2     |\n\
             +-------+-------+\n\
             | 3     | 2     |\n\
             +-------+-------+\n\
             "
        );
    }

    #[test]
    fn test_bitxor_value_type() {
        let dice = ProbabilityDistribution::new_dice(6);

        let combined_probability_distribution = dice ^ 12;

        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
             +-------+-------+\n\
             | value | count |\n\
             +=======+=======+\n\
             | 8     | 1     |\n\
             +-------+-------+\n\
             | 9     | 1     |\n\
             +-------+-------+\n\
             | 10    | 1     |\n\
             +-------+-------+\n\
             | 13    | 1     |\n\
             +-------+-------+\n\
             | 14    | 1     |\n\
             +-------+-------+\n\
             | 15    | 1     |\n\
             +-------+-------+\n\
             "
        );
    }

    #[test]
    fn test_value_type_bitxor() {
        let dice = ProbabilityDistribution::new_dice(4);

        let combined_probability_distribution = 42 ^ dice;

        assert_eq!(
            combined_probability_distribution
                .to_table()
                .to_string()
                .replace("\r\n", "\n"),
            "\
             +-------+-------+\n\
             | value | count |\n\
             +=======+=======+\n\
             | 40    | 1     |\n\
             +-------+-------+\n\
             | 41    | 1     |\n\
             +-------+-------+\n\
             | 43    | 1     |\n\
             +-------+-------+\n\
             | 46    | 1     |\n\
             +-------+-------+\n\
             "
        );
    }
}
