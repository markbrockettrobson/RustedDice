use std::collections::{HashMap};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::ops::Add;

use super::constraint::{Constraint, IsTheoreticallyPossible};

type IdToConstraintMap = HashMap<u16, Constraint>;
type IdToValueMap = HashMap<u16, i32>;

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct ConstraintMap {
    pub map: IdToConstraintMap,
}

impl IsTheoreticallyPossible for ConstraintMap {
    fn is_theoretically_possible(&self) -> bool {
        return !self.map.iter().any(|(_, constraint)| !constraint.is_theoretically_possible())
    }
}

trait CompilesWithConstraints {
    fn compiles(&self, value_map: IdToValueMap) -> bool;
}

impl CompilesWithConstraints for ConstraintMap {
    fn compiles(&self, id_value_map: IdToValueMap) -> bool {
        for (id, value) in &id_value_map {
            if self.map.contains_key(id) && !self.map[id].valid_values.contains(value) {
                return false;
            }
        }
        true
    }
}

impl Add for ConstraintMap {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut new_map = HashMap::new();
        new_map.extend(self.map.into_iter());

        for (id, value) in other.map {
            match new_map.entry(id) {
                Occupied(mut e) => {
                    let new_value = e.get().clone() + value;
                    e.insert(new_value);
                }
                Vacant(e) => {
                    e.insert(value);
                }
            }
        }
        ConstraintMap { map: new_map }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_theoretically_possible_no_constraint_true() {
        let map: HashMap<u16, Constraint> = HashMap::new();
        let constraint_map = ConstraintMap { map };
        assert!(constraint_map.is_theoretically_possible());        
    }

    #[test]
    fn is_theoretically_possible_true() {
        let constraint1_123 = Constraint {id: 1, valid_values: vec![1, 2, 3].into_iter().collect()};
        let constraint2_123 = Constraint {id: 2, valid_values: vec![1, 2, 3].into_iter().collect()};
        let constraint3_123 = Constraint {id: 3, valid_values: vec![1, 2, 3].into_iter().collect()};
        let mut map: HashMap<u16, Constraint> = HashMap::new();
        map.insert(1, constraint1_123);
        map.insert(2, constraint2_123);
        map.insert(3, constraint3_123);
        let constraint_map = ConstraintMap { map };
        assert!(constraint_map.is_theoretically_possible());        
    }

    #[test]
    fn is_theoretically_possible_one_bad_constraint_true() {
        let constraint1_123 = Constraint {id: 1, valid_values: vec![1, 2, 3].into_iter().collect()};
        let constraint2 = Constraint {id: 2, valid_values: vec![].into_iter().collect()};
        let constraint3_123 = Constraint {id: 3, valid_values: vec![1, 2, 3].into_iter().collect()};
        let mut map: HashMap<u16, Constraint> = HashMap::new();
        map.insert(1, constraint1_123);
        map.insert(2, constraint2);
        map.insert(3, constraint3_123);
        let constraint_map = ConstraintMap { map };
        assert!(!constraint_map.is_theoretically_possible());        
    }

    #[test]
    fn is_theoretically_possible_all_bad_constraint_true() {
        let constraint1 = Constraint {id: 1, valid_values: vec![].into_iter().collect()};
        let constraint2 = Constraint {id: 2, valid_values: vec![].into_iter().collect()};
        let constraint3 = Constraint {id: 3, valid_values: vec![].into_iter().collect()};
        let mut map: HashMap<u16, Constraint> = HashMap::new();
        map.insert(1, constraint1);
        map.insert(2, constraint2);
        map.insert(3, constraint3);
        let constraint_map = ConstraintMap { map };
        assert!(!constraint_map.is_theoretically_possible());        
    }

    #[test]
    fn compiles_no_constraint_map_true() {
        let map: HashMap<u16, Constraint> = HashMap::new();
        let constraint_map = ConstraintMap { map };
        assert!(constraint_map.compiles( HashMap::new() ));        
    }

    #[test]
    fn compiles_one_id_in_map_true() {
        let constraint1_123 = Constraint {id: 1, valid_values: vec![1, 2, 3].into_iter().collect()};
        let constraint2_123 = Constraint {id: 2, valid_values: vec![1, 2, 3].into_iter().collect()};
        let constraint3_123 = Constraint {id: 3, valid_values: vec![1, 2, 3].into_iter().collect()};
        let mut map: HashMap<u16, Constraint> = HashMap::new();
        map.insert(1, constraint1_123);
        map.insert(2, constraint2_123);
        map.insert(3, constraint3_123);
        
        let constraint_map = ConstraintMap { map };

        let mut id_value_map: HashMap<u16, i32> = HashMap::new();
        id_value_map.insert(1, 1);
        id_value_map.insert(7, 1);
        assert!(constraint_map.compiles(id_value_map));
    }
    
    #[test]
    fn compiles_one_id_in_map_false() {
        let constraint1_123 = Constraint {id: 1, valid_values: vec![1, 2, 3].into_iter().collect()};
        let constraint2_123 = Constraint {id: 2, valid_values: vec![1, 2, 3].into_iter().collect()};
        let constraint3_123 = Constraint {id: 3, valid_values: vec![1, 2, 3].into_iter().collect()};
        let mut map: HashMap<u16, Constraint> = HashMap::new();
        map.insert(1, constraint1_123);
        map.insert(2, constraint2_123);
        map.insert(3, constraint3_123);
        
        let constraint_map = ConstraintMap { map };

        let mut id_value_map: HashMap<u16, i32> = HashMap::new();
        id_value_map.insert(1, 4);
        id_value_map.insert(7, 1);
        assert!(!constraint_map.compiles(id_value_map));
    }
    #[test]
   
    fn compiles_more_then_one_id_in_map_true() {
        let constraint1_123 = Constraint {id: 1, valid_values: vec![1, 2, 3].into_iter().collect()};
        let constraint2_123 = Constraint {id: 2, valid_values: vec![1, 2, 3].into_iter().collect()};
        let constraint3_123 = Constraint {id: 3, valid_values: vec![1, 2, 3].into_iter().collect()};
        let mut map: HashMap<u16, Constraint> = HashMap::new();
        map.insert(1, constraint1_123);
        map.insert(2, constraint2_123);
        map.insert(3, constraint3_123);
        
        let constraint_map = ConstraintMap { map };

        let mut id_value_map: HashMap<u16, i32> = HashMap::new();
        id_value_map.insert(1, 1);
        id_value_map.insert(2, 2);
        id_value_map.insert(7, 1);
        assert!(constraint_map.compiles(id_value_map));
    }

    #[test]
    fn compiles_more_then_one_id_in_map_false() {
        let constraint1_123 = Constraint {id: 1, valid_values: vec![1, 2, 3].into_iter().collect()};
        let constraint2_123 = Constraint {id: 2, valid_values: vec![1, 2, 3].into_iter().collect()};
        let constraint3_123 = Constraint {id: 3, valid_values: vec![1, 2, 3].into_iter().collect()};
        let mut map: HashMap<u16, Constraint> = HashMap::new();
        map.insert(1, constraint1_123);
        map.insert(2, constraint2_123);
        map.insert(3, constraint3_123);
        
        let constraint_map = ConstraintMap { map };

        let mut id_value_map: HashMap<u16, i32> = HashMap::new();
        id_value_map.insert(1, 1);
        id_value_map.insert(2, 5);
        assert!(!constraint_map.compiles(id_value_map));
    }

    #[test]
    fn compiles_all_id_in_map_true() {
        let constraint1_123 = Constraint {id: 1, valid_values: vec![1, 2, 3].into_iter().collect()};
        let constraint2_123 = Constraint {id: 2, valid_values: vec![1, 2, 3].into_iter().collect()};
        let constraint3_123 = Constraint {id: 3, valid_values: vec![1, 2, 3].into_iter().collect()};
        let mut map: HashMap<u16, Constraint> = HashMap::new();
        map.insert(1, constraint1_123);
        map.insert(2, constraint2_123);
        map.insert(3, constraint3_123);
        
        let constraint_map = ConstraintMap { map };

        let mut id_value_map: HashMap<u16, i32> = HashMap::new();
        id_value_map.insert(1, 1);
        id_value_map.insert(2, 2);
        id_value_map.insert(3, 3);
        assert!(constraint_map.compiles(id_value_map));
    }
    
    #[test]
    fn compiles_all_id_in_map_false() {
        let constraint1_123 = Constraint {id: 1, valid_values: vec![1, 2, 3].into_iter().collect()};
        let constraint2_123 = Constraint {id: 2, valid_values: vec![1, 2, 3].into_iter().collect()};
        let constraint3_123 = Constraint {id: 3, valid_values: vec![1, 2, 3].into_iter().collect()};
        let mut map: HashMap<u16, Constraint> = HashMap::new();
        map.insert(1, constraint1_123);
        map.insert(2, constraint2_123);
        map.insert(3, constraint3_123);
        
        let constraint_map = ConstraintMap { map };

        let mut id_value_map: HashMap<u16, i32> = HashMap::new();
        id_value_map.insert(1, 1);
        id_value_map.insert(2, 2);
        id_value_map.insert(3, 4);
        assert!(!constraint_map.compiles(id_value_map));
    }
    
    #[test]
    fn combine_no_id_common() {
        let constraint1_123 = Constraint {id: 1, valid_values: vec![1, 2, 3].into_iter().collect()};
        let constraint2_123 = Constraint {id: 2, valid_values: vec![1, 2, 3].into_iter().collect()};
        let constraint3_123 = Constraint {id: 3, valid_values: vec![1, 2, 3].into_iter().collect()};
        let constraint4_123 = Constraint {id: 4, valid_values: vec![1, 2, 3].into_iter().collect()};
        let mut map_one: HashMap<u16, Constraint> = HashMap::new();
        map_one.insert(1, constraint1_123);
        map_one.insert(2, constraint2_123);
        let mut map_two: HashMap<u16, Constraint> = HashMap::new();
        map_two.insert(3, constraint3_123);
        map_two.insert(4, constraint4_123);
        
        let constraint_map_one = ConstraintMap { map: map_one };
        let constraint_map_two = ConstraintMap { map: map_two };
        
        let constraint_map_three = constraint_map_one + constraint_map_two;

        assert!(constraint_map_three.map.contains_key(&1));
        assert!(constraint_map_three.map.contains_key(&2));
        assert!(constraint_map_three.map.contains_key(&3));
        assert!(constraint_map_three.map.contains_key(&4));
        assert!(!constraint_map_three.map.contains_key(&5));
    }
    
    #[test]
    fn combine_one_id_common() {
        let constraint1_123 = Constraint {id: 1, valid_values: vec![1, 2, 3].into_iter().collect()};
        let constraint2_123 = Constraint {id: 2, valid_values: vec![1, 2, 3].into_iter().collect()};
        let constraint2_345 = Constraint {id: 2, valid_values: vec![3, 4, 5].into_iter().collect()};
        let constraint3_123 = Constraint {id: 3, valid_values: vec![1, 2, 3].into_iter().collect()};
        let mut map_one: HashMap<u16, Constraint> = HashMap::new();
        map_one.insert(1, constraint1_123);
        map_one.insert(2, constraint2_123);
        let mut map_two: HashMap<u16, Constraint> = HashMap::new();
        map_two.insert(2, constraint2_345);
        map_two.insert(3, constraint3_123);
        
        let constraint_map_one = ConstraintMap { map: map_one };
        let constraint_map_two = ConstraintMap { map: map_two };
        
        let constraint_map_three = constraint_map_one + constraint_map_two;

        assert!(constraint_map_three.map.contains_key(&1));
        assert!(constraint_map_three.map.contains_key(&2));
        assert!(constraint_map_three.map.contains_key(&3));
        assert!(!constraint_map_three.map.get(&2).unwrap().valid_values.contains(&1));
        assert!(!constraint_map_three.map.get(&2).unwrap().valid_values.contains(&2));
        assert!(constraint_map_three.map.get(&2).unwrap().valid_values.contains(&3));
        assert!(!constraint_map_three.map.get(&2).unwrap().valid_values.contains(&4));
        assert!(!constraint_map_three.map.get(&2).unwrap().valid_values.contains(&5));
        assert!(!constraint_map_three.map.contains_key(&4));
    }
    
    #[test]
    fn combine_all_id_common() {
        let constraint1_123 = Constraint {id: 1, valid_values: vec![1, 2, 3].into_iter().collect()};
        let constraint1_234 = Constraint {id: 1, valid_values: vec![2, 3, 4].into_iter().collect()};
        let constraint2_123 = Constraint {id: 2, valid_values: vec![1, 2, 3].into_iter().collect()};
        let constraint2_345 = Constraint {id: 2, valid_values: vec![3, 4, 5].into_iter().collect()};
        let mut map_one: HashMap<u16, Constraint> = HashMap::new();
        map_one.insert(1, constraint1_123);
        map_one.insert(2, constraint2_123);
        let mut map_two: HashMap<u16, Constraint> = HashMap::new();
        map_two.insert(1, constraint1_234);
        map_two.insert(2, constraint2_345);
        
        let constraint_map_one = ConstraintMap { map: map_one };
        let constraint_map_two = ConstraintMap { map: map_two };
        
        let constraint_map_three = constraint_map_one + constraint_map_two;

        print!("{:?}", constraint_map_three.map);
        assert!(constraint_map_three.map.contains_key(&1));
        assert!(!constraint_map_three.map.get(&1).unwrap().valid_values.contains(&1));
        assert!(constraint_map_three.map.get(&1).unwrap().valid_values.contains(&2));
        assert!(constraint_map_three.map.get(&1).unwrap().valid_values.contains(&3));
        assert!(!constraint_map_three.map.get(&1).unwrap().valid_values.contains(&4));
        assert!(constraint_map_three.map.contains_key(&2));
        assert!(!constraint_map_three.map.get(&2).unwrap().valid_values.contains(&1));
        assert!(!constraint_map_three.map.get(&2).unwrap().valid_values.contains(&2));
        assert!(constraint_map_three.map.get(&2).unwrap().valid_values.contains(&3));
        assert!(!constraint_map_three.map.get(&2).unwrap().valid_values.contains(&4));
        assert!(!constraint_map_three.map.get(&2).unwrap().valid_values.contains(&5));
        assert!(!constraint_map_three.map.contains_key(&4));
    }
    
        
    #[test]
    fn combine_no_impossable_options_common() {
        let constraint1_123 = Constraint {id: 1, valid_values: vec![1, 2, 3].into_iter().collect()};
        let constraint1_234 = Constraint {id: 1, valid_values: vec![2, 3, 4].into_iter().collect()};
        let constraint2_123 = Constraint {id: 2, valid_values: vec![1, 2, 3].into_iter().collect()};
        let constraint2_345 = Constraint {id: 2, valid_values: vec![3, 4, 5].into_iter().collect()};
        let mut map_one: HashMap<u16, Constraint> = HashMap::new();
        map_one.insert(1, constraint1_123);
        map_one.insert(2, constraint2_123);
        let mut map_two: HashMap<u16, Constraint> = HashMap::new();
        map_two.insert(1, constraint1_234);
        map_two.insert(2, constraint2_345);
        
        let constraint_map_one = ConstraintMap { map: map_one };
        let constraint_map_two = ConstraintMap { map: map_two };
        
        let constraint_map_three = constraint_map_one + constraint_map_two;

        print!("{:?}", constraint_map_three.map);
        assert!(constraint_map_three.map.contains_key(&1));
        assert!(!constraint_map_three.map.get(&1).unwrap().valid_values.contains(&1));
        assert!(constraint_map_three.map.get(&1).unwrap().valid_values.contains(&2));
        assert!(constraint_map_three.map.get(&1).unwrap().valid_values.contains(&3));
        assert!(!constraint_map_three.map.get(&1).unwrap().valid_values.contains(&4));
        assert!(constraint_map_three.map.contains_key(&2));
        assert!(!constraint_map_three.map.get(&2).unwrap().valid_values.contains(&1));
        assert!(!constraint_map_three.map.get(&2).unwrap().valid_values.contains(&2));
        assert!(constraint_map_three.map.get(&2).unwrap().valid_values.contains(&3));
        assert!(!constraint_map_three.map.get(&2).unwrap().valid_values.contains(&4));
        assert!(!constraint_map_three.map.get(&2).unwrap().valid_values.contains(&5));
        assert!(!constraint_map_three.map.contains_key(&4));
    }

    #[test]
    fn test_fmt() {
        let constraint1_123 = Constraint {id: 1, valid_values: vec![1, 2, 3].into_iter().collect()};
        let constraint2_123 = Constraint {id: 2, valid_values: vec![1, 2, 3].into_iter().collect()};
        let mut map: HashMap<u16, Constraint> = HashMap::new();
        map.insert(1, constraint1_123);
        map.insert(2, constraint2_123);
        
        let constraint_map = ConstraintMap { map: map.clone() };
        assert_eq!(format!("{constraint_map:?}"), format!("ConstraintMap {{ map: {:?} }}", map));
    }
}
