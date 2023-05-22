use crate::constraint_management::{
    Constraint, ConstraintValueType, IsConstraintCompiledWith, IsTheoreticallyPossible,
};

impl IsConstraintCompiledWith for Constraint {
    fn is_compliant_with(&self, value: ConstraintValueType) -> bool {
        self.valid_values.contains(&value)
    }
}

impl IsTheoreticallyPossible for Constraint {
    fn is_theoretically_possible(&self) -> bool {
        !self.valid_values.is_empty()
    }
}

#[cfg(test)]
mod tests {
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
