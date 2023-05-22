use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

use crate::constraint_management::{Constraint, ConstraintIdType, ConstraintMap};

#[allow(dead_code)]
impl ConstraintMap {
    pub(crate) fn new_empty_constraint_map() -> ConstraintMap {
        let map: HashMap<ConstraintIdType, Constraint> = HashMap::new();
        ConstraintMap { map }
    }

    pub(crate) fn new_single_constraint_constraint_map(constraint: Constraint) -> ConstraintMap {
        let mut map: HashMap<ConstraintIdType, Constraint> = HashMap::new();
        map.insert(constraint.id, constraint);
        ConstraintMap { map }
    }

    pub(crate) fn new_constraint_map(
        constraints: impl IntoIterator<Item = Constraint>,
    ) -> ConstraintMap {
        let mut map: HashMap<ConstraintIdType, Constraint> = HashMap::new();
        for constraint in constraints {
            match map.entry(constraint.id) {
                Occupied(mut e) => {
                    let new_constraint = e.get().clone() + constraint;
                    e.insert(new_constraint);
                }
                Vacant(e) => {
                    e.insert(constraint);
                }
            }
        }
        ConstraintMap { map }
    }
}

#[cfg(test)]
mod tests {
    use crate::constraint_management::{Constraint, IdToConstraintMap};

    use super::*;
    use std::vec::IntoIter;

    #[test]
    fn test_new_empty_constraint_map() {
        let map: IdToConstraintMap = HashMap::new();
        let constraint_map = ConstraintMap { map };
        assert_eq!(constraint_map, ConstraintMap::new_empty_constraint_map());
    }

    #[test]
    fn test_new_constraint_map() {
        let constraint3_123 = Constraint::new_many_item_constraint(3, vec![1, 2, 3]);
        let mut map: IdToConstraintMap = HashMap::new();
        map.insert(3, constraint3_123.clone());
        let constraint_map = ConstraintMap { map };
        assert_eq!(
            constraint_map,
            ConstraintMap::new_single_constraint_constraint_map(constraint3_123)
        );
    }

    #[test]
    fn test_new_constraint_map_no_constraint() {
        let constraint_iter: IntoIter<Constraint> = vec![].into_iter();

        let map: IdToConstraintMap = HashMap::new();
        let constraint_map = ConstraintMap { map };
        assert_eq!(
            constraint_map,
            ConstraintMap::new_constraint_map(constraint_iter)
        );
    }

    #[test]
    fn test_new_constraint_map_single_constraint() {
        let constraint3_123 = Constraint::new_many_item_constraint(3, vec![1, 2, 3]);
        let constraint_iter = vec![constraint3_123.clone()];

        let mut map: IdToConstraintMap = HashMap::new();
        map.insert(3, constraint3_123);
        let constraint_map = ConstraintMap { map };
        assert_eq!(
            constraint_map,
            ConstraintMap::new_constraint_map(constraint_iter)
        );
    }

    #[test]
    fn test_new_constraint_map_constraints() {
        let constraint1_123 = Constraint::new_many_item_constraint(1, vec![1, 2, 3]);
        let constraint2_123 = Constraint::new_many_item_constraint(2, vec![1, 2, 3]);
        let constraint3_123 = Constraint::new_many_item_constraint(3, vec![1, 2, 3]);
        let constraint_iter = vec![
            constraint1_123.clone(),
            constraint2_123.clone(),
            constraint3_123.clone(),
        ]
        .into_iter();

        let mut map: HashMap<u16, Constraint> = HashMap::new();
        map.insert(1, constraint1_123);
        map.insert(2, constraint2_123);
        map.insert(3, constraint3_123);
        let constraint_map = ConstraintMap { map };
        assert_eq!(
            constraint_map,
            ConstraintMap::new_constraint_map(constraint_iter)
        );
    }

    #[test]
    fn test_new_constraint_map_constraints_some_overlap() {
        let constraint1_123 = Constraint::new_many_item_constraint(1, vec![1, 2, 3]);
        let constraint2_123 = Constraint::new_many_item_constraint(2, vec![1, 2, 3]);
        let constraint2_23 = Constraint::new_many_item_constraint(2, vec![2, 3]);
        let constraint2_234 = Constraint::new_many_item_constraint(2, vec![2, 3, 4]);
        let constraint3_123 = Constraint::new_many_item_constraint(3, vec![1, 2, 3]);
        let constraint_iter = vec![
            constraint1_123.clone(),
            constraint2_123,
            constraint2_234,
            constraint3_123.clone(),
        ]
        .into_iter();

        let mut map: HashMap<u16, Constraint> = HashMap::new();
        map.insert(1, constraint1_123);
        map.insert(2, constraint2_23);
        map.insert(3, constraint3_123);
        let constraint_map = ConstraintMap { map };
        assert_eq!(
            constraint_map,
            ConstraintMap::new_constraint_map(constraint_iter)
        );
    }
}
