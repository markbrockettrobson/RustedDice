use std::collections::BTreeMap;
use std::ops::Not;

use crate::{
    probability::{ProbabilityDistribution, ProbabilityOutcome},
    CountType,
};

use super::add_outcome_to_map;

impl Not for ProbabilityDistribution {
    type Output = Self;

    /// Implements the bitwise not operator for [ProbabilityDistribution].
    /// The values of the distribution after the bitwise not.
    ///
    /// # Arguments
    ///
    /// * `self` - The [ProbabilityDistribution].
    ///
    /// # Returns
    ///
    /// The resulting [ProbabilityDistribution] after the bitwise not operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::probability::ProbabilityDistribution;
    /// # use crate::rusted_dice::probability::ToTable;
    ///let dice = ProbabilityDistribution::new_dice(4);
    ///
    ///assert_eq!(
    ///    (!dice)
    ///        .to_table()
    ///        .to_string()
    ///        .replace("\r\n", "\n"),
    ///     "\
    ///     +-------+-------+\n\
    ///     | value | count |\n\
    ///     +=======+=======+\n\
    ///     | -5    | 1     |\n\
    ///     +-------+-------+\n\
    ///     | -4    | 1     |\n\
    ///     +-------+-------+\n\
    ///     | -3    | 1     |\n\
    ///     +-------+-------+\n\
    ///     | -2    | 1     |\n\
    ///     +-------+-------+\n\
    ///     ");
    /// ```
    #[cfg_attr(test, mutants::skip)]
    // This is skipped because / -1 is equivalent to * -1
    fn not(self) -> Self {
        let mut new_outcome_counts: BTreeMap<ProbabilityOutcome, CountType> = BTreeMap::new();

        for (value, count) in self.outcome_counts.iter() {
            let new_value = !value.clone();
            add_outcome_to_map(&mut new_outcome_counts, new_value, *count);
        }
        ProbabilityDistribution {
            outcome_counts: new_outcome_counts,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::probability::ProbabilityDistribution;
    use crate::probability::ToTable;

    #[test]
    fn test_neg() {
        let dice = ProbabilityDistribution::new_dice(10);

        assert_eq!(
            (!dice).to_table().to_string().replace("\r\n", "\n"),
            "\
            +-------+-------+\n\
            | value | count |\n\
            +=======+=======+\n\
            | -11   | 1     |\n\
            +-------+-------+\n\
            | -10   | 1     |\n\
            +-------+-------+\n\
            | -9    | 1     |\n\
            +-------+-------+\n\
            | -8    | 1     |\n\
            +-------+-------+\n\
            | -7    | 1     |\n\
            +-------+-------+\n\
            | -6    | 1     |\n\
            +-------+-------+\n\
            | -5    | 1     |\n\
            +-------+-------+\n\
            | -4    | 1     |\n\
            +-------+-------+\n\
            | -3    | 1     |\n\
            +-------+-------+\n\
            | -2    | 1     |\n\
            +-------+-------+\n\
            "
        );
    }
}
