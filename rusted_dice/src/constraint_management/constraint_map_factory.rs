use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::vec::IntoIter;

use super::constraint::Constraint;
use super::constraint_map::ConstraintMap;

#[allow(dead_code)]
pub(crate) struct ConstraintMapFactory;

#[allow(dead_code)]
impl ConstraintMapFactory {
    pub(crate) fn new_empty_constraint_map() -> ConstraintMap {
        let map: HashMap<u16, Constraint> = HashMap::new();
        ConstraintMap { map }
    }

    pub(crate) fn single_constraint_constraint_map(constraint: Constraint) -> ConstraintMap {
        let mut map: HashMap<u16, Constraint> = HashMap::new();
        map.insert(constraint.id, constraint);
        ConstraintMap { map }
    }

    pub(crate) fn new_constraint_map(constraints: IntoIter<Constraint>) -> ConstraintMap {
        let mut map: HashMap<u16, Constraint> = HashMap::new();
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
    use super::*;

    #[test]
    fn test_new_empty_constraint_map() {
        let map: HashMap<u16, Constraint> = HashMap::new();
        let constraint_map = ConstraintMap { map };
        assert_eq!(
            constraint_map,
            ConstraintMapFactory::new_empty_constraint_map()
        );
    }

    #[test]
    fn test_new_constraint_map() {
        let constraint3_123 = Constraint {
            id: 3,
            valid_values: vec![1, 2, 3].into_iter().collect(),
        };
        let mut map: HashMap<u16, Constraint> = HashMap::new();
        map.insert(3, constraint3_123.clone());
        let constraint_map = ConstraintMap { map };
        assert_eq!(
            constraint_map,
            ConstraintMapFactory::single_constraint_constraint_map(constraint3_123)
        );
    }

    #[test]
    fn test_new_constraint_map_no_constraint() {
        let constraint_iter: IntoIter<Constraint> = vec![].into_iter();

        let map: HashMap<u16, Constraint> = HashMap::new();
        let constraint_map = ConstraintMap { map };
        assert_eq!(
            constraint_map,
            ConstraintMapFactory::new_constraint_map(constraint_iter)
        );
    }

    #[test]
    fn test_new_constraint_map_single_constraint() {
        let constraint3_123 = Constraint {
            id: 3,
            valid_values: vec![1, 2, 3].into_iter().collect(),
        };
        let constraint_iter = vec![constraint3_123.clone()].into_iter();

        let mut map: HashMap<u16, Constraint> = HashMap::new();
        map.insert(3, constraint3_123);
        let constraint_map = ConstraintMap { map };
        assert_eq!(
            constraint_map,
            ConstraintMapFactory::new_constraint_map(constraint_iter)
        );
    }

    #[test]
    fn test_new_constraint_map_constraints() {
        let constraint1_123 = Constraint {
            id: 1,
            valid_values: vec![1, 2, 3].into_iter().collect(),
        };
        let constraint2_123 = Constraint {
            id: 2,
            valid_values: vec![1, 2, 3].into_iter().collect(),
        };
        let constraint3_123 = Constraint {
            id: 3,
            valid_values: vec![1, 2, 3].into_iter().collect(),
        };
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
            ConstraintMapFactory::new_constraint_map(constraint_iter)
        );
    }

    #[test]
    fn test_new_constraint_map_constraints_some_overlap() {
        let constraint1_123 = Constraint {
            id: 1,
            valid_values: vec![1, 2, 3].into_iter().collect(),
        };
        let constraint2_123 = Constraint {
            id: 2,
            valid_values: vec![1, 2, 3].into_iter().collect(),
        };
        let constraint2_23 = Constraint {
            id: 2,
            valid_values: vec![2, 3].into_iter().collect(),
        };
        let constraint2_234 = Constraint {
            id: 2,
            valid_values: vec![2, 3, 4].into_iter().collect(),
        };
        let constraint3_123 = Constraint {
            id: 3,
            valid_values: vec![1, 2, 3].into_iter().collect(),
        };
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
            ConstraintMapFactory::new_constraint_map(constraint_iter)
        );
    }
}
