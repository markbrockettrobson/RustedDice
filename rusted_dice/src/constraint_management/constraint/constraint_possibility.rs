use crate::constraint_management::{Constraint, ConstraintValueType};

pub trait IsConstraintCompiledWith {
    fn is_compiled_with(&self, value: ConstraintValueType) -> bool;
}

impl IsConstraintCompiledWith for Constraint {
    fn is_compiled_with(&self, value: ConstraintValueType) -> bool {
        self.valid_values.contains(&value)
    }
}

pub trait IsTheoreticallyPossible {
    fn is_theoretically_possible(&self) -> bool;
}

impl IsTheoreticallyPossible for Constraint {
    fn is_theoretically_possible(&self) -> bool {
        !self.valid_values.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::constraint_management::ConstraintFactory;

    use super::*;

    #[test]
    fn is_theoretically_possible_true() {
        let constraint = ConstraintFactory::new_many_item_constraint(0, vec![1, 2, 3]);
        assert!(constraint.is_theoretically_possible());
    }

    #[test]
    fn is_theoretically_possible_false() {
        let constraint = ConstraintFactory::new_empty_constraint(0);
        assert!(!constraint.is_theoretically_possible());
    }

    #[test]
    fn is_compiled_with_true() {
        let constraint = ConstraintFactory::new_many_item_constraint(0, vec![1, 2, 3]);
        assert!(constraint.is_compiled_with(1));
        assert!(constraint.is_compiled_with(2));
        assert!(constraint.is_compiled_with(3));
    }

    #[test]
    fn is_compiled_with_false() {
        let constraint = ConstraintFactory::new_many_item_constraint(0, vec![4, 5, 6]);
        assert!(!constraint.is_compiled_with(1));
        assert!(!constraint.is_compiled_with(2));
        assert!(!constraint.is_compiled_with(3));
    }
}
