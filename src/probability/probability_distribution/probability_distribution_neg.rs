use std::ops::Neg;

use crate::probability::ProbabilityDistribution;

impl Neg for ProbabilityDistribution {
    type Output = Self;

    /// Implements the negation operator for [ProbabilityDistribution].
    /// The values of the distribution are negated.
    ///
    /// # Arguments
    ///
    /// * `self` - The [ProbabilityDistribution].
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityDistribution] after the negation operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ToTable;
    /// let dice_one = ProbabilityDistribution::new_dice(3);
    /// let dice_two = ProbabilityDistribution::new_dice(-3);
    ///
    /// assert_eq!((-dice_one).to_table(), dice_two.to_table());
    /// ```
    #[cfg_attr(test, mutants::skip)]
    // This is skipped because / -1 is equivalent to * -1
    fn neg(self) -> Self {
        self * -1
    }
}

#[cfg(test)]
mod tests {
    use crate::probability::ProbabilityDistribution;
    use crate::probability::ToTable;

    #[test]
    fn test_neg() {
        let dice_one = ProbabilityDistribution::new_dice(123);
        let dice_two = ProbabilityDistribution::new_dice(-123);

        assert_eq!((-dice_one).to_table(), dice_two.to_table());
    }
}
