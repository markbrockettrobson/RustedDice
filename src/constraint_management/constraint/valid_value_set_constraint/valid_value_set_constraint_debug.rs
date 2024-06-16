use core::fmt::Debug;
use std::hash::Hash;

#[allow(unused_imports)] // ConstraintIdType is used in docs
use crate::constraint_management::{ConstraintIdType, ValidValueSetConstraint};

impl<T> Debug for ValidValueSetConstraint<T>
where
    T: Eq + Hash + Debug + Ord,
{
    /// Formats the [ValidValueSetConstraint] as a string.
    ///
    /// The [ValidValueSetConstraint] is formatted as a struct with the following fields:
    /// - `id` - The [ConstraintIdType] for the [ValidValueSetConstraint].
    /// - `valid_values` - The valid values for the [ValidValueSetConstraint].
    ///  The valid values are sorted in ascending order.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sorted_valid_values: Vec<&T> = self.valid_values.iter().collect();
        sorted_valid_values.sort();
        write!(
            f,
            "ValidValueSetConstraint {{ id: {}, valid_values: {:?} }}",
            self.id, sorted_valid_values
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use proptest::prelude::*;

    use crate::{
        constraint_management::ConstraintIdType, proptest_strategy::TestValueTypeEnum,
        test_value_strategy, test_vec_value_strategy,
    };

    proptest! {
        #[test]
        fn test_fmt_empty(
            test_id: ConstraintIdType
        ) {
            let constraint: ValidValueSetConstraint<u8> = ValidValueSetConstraint::new_empty_constraint(test_id);
            assert_eq!(format!("{constraint:?}"), format!("ValidValueSetConstraint {{ id: {}, valid_values: [] }}", test_id));
        }

        #[test]
        fn test_fmt_single_value(
            test_id: ConstraintIdType,
            test_valid_value in test_value_strategy()
        ) {
            let constraint = ValidValueSetConstraint::new_single_valid_value_constraint(test_id, &test_valid_value);
            assert_eq!(format!("{constraint:?}"), format!("ValidValueSetConstraint {{ id: {}, valid_values: [{:?}] }}", test_id, test_valid_value));
        }

        #[test]
        fn test_fmt_many_values(
            test_id: ConstraintIdType,
            test_valid_values in test_vec_value_strategy(1, 10)
        ) {
            let constraint = ValidValueSetConstraint::new_many_item_constraint(test_id, test_valid_values.clone());
            let valid_values_set: HashSet<TestValueTypeEnum> = test_valid_values.into_iter().collect();
            let mut sorted_valid_values: Vec<&_> = valid_values_set.iter().collect();
            sorted_valid_values.sort();
            assert_eq!(format!("{constraint:?}"), format!("ValidValueSetConstraint {{ id: {}, valid_values: {:?} }}", test_id, sorted_valid_values));
        }
    }
}
