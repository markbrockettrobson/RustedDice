use crate::constraint_management::{
    AreConstraintsCompiledWith, ConstraintMap, IdToValueMap, IsTheoreticallyPossible,
};

impl IsTheoreticallyPossible for ConstraintMap {
    /// Checks if the `ConstraintMap` is theoretically possible.
    ///
    /// # Arguments
    ///
    /// * `self` - The `ConstraintMap` instance.
    ///
    /// # Returns
    ///
    /// `true` if all constraints are theoretically possible
    ///
    /// # Examples
    /// ```
    /// # use std::collections::HashMap;
    /// # use crate::rusted_dice::constraint_management::traits::IsTheoreticallyPossible;
    /// # use crate::rusted_dice::constraint_management::constraint::Constraint;
    /// # use crate::rusted_dice::constraint_management::constraint_map::ConstraintMap;
    /// # use crate::rusted_dice::constraint_management::IdToValueMap;
    /// let constraint_map_one = ConstraintMap::new_constraint_map(
    ///     vec![
    ///        Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
    ///        Constraint::new_many_item_constraint(2, vec![1, 2, 3])
    ///     ]
    /// );
    /// let constraint_map_two = ConstraintMap::new_constraint_map(
    ///     vec![
    ///        Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
    ///        Constraint::new_empty_constraint(2)
    ///     ]
    /// );
    /// assert!(constraint_map_one.is_theoretically_possible());
    /// assert!(!constraint_map_two.is_theoretically_possible());
    /// ```
    fn is_theoretically_possible(&self) -> bool {
        return !self
            .map
            .iter()
            .any(|(_, constraint)| !constraint.is_theoretically_possible());
    }
}

impl AreConstraintsCompiledWith for ConstraintMap {
    /// Checks if the `ConstraintMap` is compliant with a specific ConstraintID -> Value map.
    ///
    /// # Arguments
    ///
    /// * `self` - The `ConstraintMap` instance.
    /// * `id_value_map` - The  ConstraintID -> `ValueType` map to check compliance with.
    ///
    /// # Returns
    ///
    /// `true` if the the given `value` is a valid value `Constraint`, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// # use std::collections::HashMap;
    /// # use crate::rusted_dice::constraint_management::traits::AreConstraintsCompiledWith;
    /// # use crate::rusted_dice::constraint_management::constraint::Constraint;
    /// # use crate::rusted_dice::constraint_management::constraint_map::ConstraintMap;
    /// # use crate::rusted_dice::constraint_management::IdToValueMap;
    /// let constraint_map_one = ConstraintMap::new_constraint_map(
    ///     vec![
    ///        Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
    ///        Constraint::new_many_item_constraint(2, vec![1, 2, 3])
    ///     ]
    /// );
    /// let mut id_value_map: IdToValueMap = HashMap::new();
    /// id_value_map.insert(1, 2);
    /// id_value_map.insert(2, 3);
    /// id_value_map.insert(4, 10);
    /// assert!(constraint_map_one.is_compliant_with(id_value_map.clone()));
    /// id_value_map.insert(2, 4);
    /// assert!(!constraint_map_one.is_compliant_with(id_value_map));
    /// ```
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

    use crate::constraint_management::{Constraint, ConstraintMap};

    use super::*;

    #[test]
    fn is_theoretically_possible_no_constraint_true() {
        let constraint_map = ConstraintMap::new_empty_constraint_map();
        assert!(constraint_map.is_theoretically_possible());
    }

    #[test]
    fn is_theoretically_possible_true() {
        let constraint_map = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);
        assert!(constraint_map.is_theoretically_possible());
    }

    #[test]
    fn is_theoretically_possible_one_bad_constraint_false() {
        let constraint_map = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_empty_constraint(2),
            Constraint::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);
        assert!(!constraint_map.is_theoretically_possible());
    }

    #[test]
    fn is_theoretically_possible_all_bad_constraint_false() {
        let constraint_map = ConstraintMap::new_constraint_map(vec![
            Constraint::new_empty_constraint(1),
            Constraint::new_empty_constraint(2),
            Constraint::new_empty_constraint(3),
        ]);
        assert!(!constraint_map.is_theoretically_possible());
    }

    #[test]
    fn compiles_no_constraint_map_true() {
        let constraint_map = ConstraintMap::new_empty_constraint_map();
        assert!(constraint_map.is_compliant_with(HashMap::new()));
    }

    #[test]
    fn compiles_one_id_in_map_true() {
        let constraint_map = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);
        let mut id_value_map: IdToValueMap = HashMap::new();
        id_value_map.insert(1, 1);
        id_value_map.insert(7, 1);
        assert!(constraint_map.is_compliant_with(id_value_map));
    }

    #[test]
    fn compiles_one_id_in_map_false() {
        let constraint_map = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);
        let mut id_value_map: IdToValueMap = HashMap::new();
        id_value_map.insert(1, 4);
        id_value_map.insert(7, 1);
        assert!(!constraint_map.is_compliant_with(id_value_map));
    }

    #[test]
    fn compiles_more_then_one_id_in_map_true() {
        let constraint_map = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);

        let mut id_value_map: IdToValueMap = HashMap::new();
        id_value_map.insert(1, 1);
        id_value_map.insert(2, 2);
        id_value_map.insert(7, 1);
        assert!(constraint_map.is_compliant_with(id_value_map));
    }

    #[test]
    fn compiles_more_then_one_id_in_map_false() {
        let constraint_map = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);

        let mut id_value_map: IdToValueMap = HashMap::new();
        id_value_map.insert(1, 1);
        id_value_map.insert(2, 5);
        assert!(!constraint_map.is_compliant_with(id_value_map));
    }

    #[test]
    fn compiles_all_id_in_map_true() {
        let constraint_map = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);

        let mut id_value_map: IdToValueMap = HashMap::new();
        id_value_map.insert(1, 1);
        id_value_map.insert(2, 2);
        id_value_map.insert(3, 3);
        assert!(constraint_map.is_compliant_with(id_value_map));
    }

    #[test]
    fn compiles_all_id_in_map_false() {
        let constraint_map = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);

        let mut id_value_map: IdToValueMap = HashMap::new();
        id_value_map.insert(1, 1);
        id_value_map.insert(2, 2);
        id_value_map.insert(3, 4);
        assert!(!constraint_map.is_compliant_with(id_value_map));
    }
}
