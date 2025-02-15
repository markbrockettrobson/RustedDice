use std::cmp::Eq;
use std::fmt::Debug;
use std::hash::Hash;

use crate::constraint_management::constraint::ValidValueSetConstraint;
use crate::constraint_management::{IsConstraintCompiledWith, IsTheoreticallyPossible};

impl<T> IsConstraintCompiledWith<T> for ValidValueSetConstraint<T>
where
    T: Eq + Hash + Debug + Copy + Ord + 'static,
{
    /// Checks if the [ValidValueSetConstraint] is compliant with a specific `value`.
    ///
    /// # Arguments
    ///
    /// * `self` - The [ValidValueSetConstraint] instance.
    /// * `value` - The value (of any type) to check compliance with.
    ///
    /// # Returns
    ///
    /// `true` if the the given `value` is a valid value [ValidValueSetConstraint], `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// # use crate::rusted_dice::constraint_management::constraint::ValidValueSetConstraint;
    /// # use crate::rusted_dice::constraint_management::IsConstraintCompiledWith;
    /// let constraint = ValidValueSetConstraint::new_many_item_constraint(1, vec![1, 2, 5]);
    /// assert!(constraint.is_compliant_with(1));
    /// assert!(constraint.is_compliant_with(2));
    /// assert!(constraint.is_compliant_with(5));
    /// assert!(!constraint.is_compliant_with(6));
    /// ```
    fn is_compliant_with(&self, value: T) -> bool {
        self.valid_values.contains(&value)
    }
}

impl<T> IsTheoreticallyPossible for ValidValueSetConstraint<T>
where
    T: Eq + Hash + Ord + Debug + Copy,
{
    /// Checks if the [ValidValueSetConstraint] is theoretically possible.
    ///
    /// # Arguments
    ///
    /// * `self` - The [ValidValueSetConstraint] instance.
    ///
    /// # Returns
    ///
    /// `true` if the [ValidValueSetConstraint] is theoretically possible, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// # use crate::rusted_dice::constraint_management::constraint::ValidValueSetConstraint;
    /// # use crate::rusted_dice::constraint_management::IsTheoreticallyPossible;
    /// let constraint_one = ValidValueSetConstraint::new_many_item_constraint(1, vec![1, 2, 5]);
    /// let constraint_two = ValidValueSetConstraint::<char>::new_empty_constraint(1);
    /// assert!(constraint_one.is_theoretically_possible());
    /// assert!(!constraint_two.is_theoretically_possible());
    /// ```
    fn is_theoretically_possible(&self) -> bool {
        !self.valid_values.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_theoretically_possible_true() {
        let constraint = ValidValueSetConstraint::new_many_item_constraint(0, vec![1, 2, 3]);
        assert!(constraint.is_theoretically_possible());
    }

    #[test]
    fn is_theoretically_possible_false() {
        let constraint = ValidValueSetConstraint::<char>::new_empty_constraint(0);
        assert!(!constraint.is_theoretically_possible());
    }

    #[test]
    fn is_compiled_with_true() {
        let constraint = ValidValueSetConstraint::new_many_item_constraint(0, vec![1, 2, 3]);
        assert!(constraint.is_compliant_with(1));
        assert!(constraint.is_compliant_with(2));
        assert!(constraint.is_compliant_with(3));
    }

    #[test]
    fn is_compiled_with_false() {
        let constraint = ValidValueSetConstraint::new_many_item_constraint(0, vec![4, 5, 6]);
        assert!(!constraint.is_compliant_with(1));
        assert!(!constraint.is_compliant_with(2));
        assert!(!constraint.is_compliant_with(3));
    }
}
