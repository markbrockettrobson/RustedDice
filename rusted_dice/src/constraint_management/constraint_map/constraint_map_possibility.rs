use crate::constraint_management::{
    AreConstraintsCompiledWith, ConstraintMap, IdToValueMap, IsTheoreticallyPossible,
};

impl IsTheoreticallyPossible for ConstraintMap {
    fn is_theoretically_possible(&self) -> bool {
        return !self
            .map
            .iter()
            .any(|(_, constraint)| !constraint.is_theoretically_possible());
    }
}

impl AreConstraintsCompiledWith for ConstraintMap {
    fn is_compliant_with(&self, id_value_map: IdToValueMap) -> bool {
        for (id, value) in &id_value_map {
            if self.map.contains_key(id) && !self.map[id].valid_values.contains(value) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::constraint_management::{ConstraintFactory, ConstraintMapFactory};

    use super::*;

    #[test]
    fn is_theoretically_possible_no_constraint_true() {
        let constraint_map = ConstraintMapFactory::new_empty_constraint_map();
        assert!(constraint_map.is_theoretically_possible());
    }

    #[test]
    fn is_theoretically_possible_true() {
        let constraint_map = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(1, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(2, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);
        assert!(constraint_map.is_theoretically_possible());
    }

    #[test]
    fn is_theoretically_possible_one_bad_constraint_false() {
        let constraint_map = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(1, vec![1, 2, 3]),
            ConstraintFactory::new_empty_constraint(2),
            ConstraintFactory::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);
        assert!(!constraint_map.is_theoretically_possible());
    }

    #[test]
    fn is_theoretically_possible_all_bad_constraint_false() {
        let constraint_map = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_empty_constraint(1),
            ConstraintFactory::new_empty_constraint(2),
            ConstraintFactory::new_empty_constraint(3),
        ]);
        assert!(!constraint_map.is_theoretically_possible());
    }

    #[test]
    fn compiles_no_constraint_map_true() {
        let constraint_map = ConstraintMapFactory::new_empty_constraint_map();
        assert!(constraint_map.is_compliant_with(HashMap::new()));
    }

    #[test]
    fn compiles_one_id_in_map_true() {
        let constraint_map = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(1, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(2, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);
        let mut id_value_map: IdToValueMap = HashMap::new();
        id_value_map.insert(1, 1);
        id_value_map.insert(7, 1);
        assert!(constraint_map.is_compliant_with(id_value_map));
    }

    #[test]
    fn compiles_one_id_in_map_false() {
        let constraint_map = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(1, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(2, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);
        let mut id_value_map: IdToValueMap = HashMap::new();
        id_value_map.insert(1, 4);
        id_value_map.insert(7, 1);
        assert!(!constraint_map.is_compliant_with(id_value_map));
    }

    #[test]
    fn compiles_more_then_one_id_in_map_true() {
        let constraint_map = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(1, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(2, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);

        let mut id_value_map: IdToValueMap = HashMap::new();
        id_value_map.insert(1, 1);
        id_value_map.insert(2, 2);
        id_value_map.insert(7, 1);
        assert!(constraint_map.is_compliant_with(id_value_map));
    }

    #[test]
    fn compiles_more_then_one_id_in_map_false() {
        let constraint_map = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(1, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(2, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);

        let mut id_value_map: IdToValueMap = HashMap::new();
        id_value_map.insert(1, 1);
        id_value_map.insert(2, 5);
        assert!(!constraint_map.is_compliant_with(id_value_map));
    }

    #[test]
    fn compiles_all_id_in_map_true() {
        let constraint_map = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(1, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(2, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);

        let mut id_value_map: IdToValueMap = HashMap::new();
        id_value_map.insert(1, 1);
        id_value_map.insert(2, 2);
        id_value_map.insert(3, 3);
        assert!(constraint_map.is_compliant_with(id_value_map));
    }

    #[test]
    fn compiles_all_id_in_map_false() {
        let constraint_map = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(1, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(2, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);

        let mut id_value_map: IdToValueMap = HashMap::new();
        id_value_map.insert(1, 1);
        id_value_map.insert(2, 2);
        id_value_map.insert(3, 4);
        assert!(!constraint_map.is_compliant_with(id_value_map));
    }
}
