use crate::{
    constraint_management::{Constraint, IsConstraintCompiledWith, IsTheoreticallyPossible},
    ValueType,
};

impl IsConstraintCompiledWith for Constraint {
    /// Checks if the `Constraint` is compliant with a specific `ValueType`.
    ///
    /// # Arguments
    ///
    /// * `self` - The `Constraint` instance.
    /// * `value` - The `ValueType` to check compliance with.
    ///
    /// # Returns
    ///
    /// `true` if the the given `value` is a valid value `Constraint`, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// # use crate::rusted_dice::constraint_management::traits::IsConstraintCompiledWith;
    /// # use crate::rusted_dice::constraint_management::constraint::Constraint;
    /// let constraint = Constraint::new_many_item_constraint(1, vec![1, 2, 5]);
    /// assert!(constraint.is_compliant_with(1));
    /// assert!(constraint.is_compliant_with(2));
    /// assert!(constraint.is_compliant_with(5));
    /// assert!(!constraint.is_compliant_with(6));
    /// ```
    fn is_compliant_with(&self, value: ValueType) -> bool {
        self.valid_values.contains(&value)
    }
}

impl IsTheoreticallyPossible for Constraint {
    /// Checks if the `Constraint` is theoretically possible.
    ///
    /// # Arguments
    ///
    /// * `self` - The `Constraint` instance.
    ///
    /// # Returns
    ///
    /// `true` if the `Constraint` is theoretically possible, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// # use crate::rusted_dice::constraint_management::traits::IsTheoreticallyPossible;
    /// # use crate::rusted_dice::constraint_management::constraint::Constraint;
    /// let constraint_one = Constraint::new_many_item_constraint(1, vec![1, 2, 5]);
    /// let constraint_two = Constraint::new_empty_constraint(1);
    /// assert!(constraint_one.is_theoretically_possible());
    /// assert!(!constraint_two.is_theoretically_possible());
    /// ```
    fn is_theoretically_possible(&self) -> bool {
        !self.valid_values.is_empty()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn is_theoretically_possible_true() {
        let constraint = Constraint::new_many_item_constraint(0, vec![1, 2, 3]);
        assert!(constraint.is_theoretically_possible());
    }

    #[test]
    fn is_theoretically_possible_false() {
        let constraint = Constraint::new_empty_constraint(0);
        assert!(!constraint.is_theoretically_possible());
    }

    #[test]
    fn is_compiled_with_true() {
        let constraint = Constraint::new_many_item_constraint(0, vec![1, 2, 3]);
        assert!(constraint.is_compliant_with(1));
        assert!(constraint.is_compliant_with(2));
        assert!(constraint.is_compliant_with(3));
    }

    #[test]
    fn is_compiled_with_false() {
        let constraint = Constraint::new_many_item_constraint(0, vec![4, 5, 6]);
        assert!(!constraint.is_compliant_with(1));
        assert!(!constraint.is_compliant_with(2));
        assert!(!constraint.is_compliant_with(3));
    }
}
