use std::cmp::Ordering;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::ops::Add;

use crate::constraint_management::{
    Constraint, ConstraintIDType, ConstraintValueType, IsTheoreticallyPossible,
};

type IdToConstraintMap = HashMap<ConstraintIDType, Constraint>;
type IdToValueMap = HashMap<ConstraintIDType, ConstraintValueType>;

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConstraintMap {
    pub map: IdToConstraintMap,
}

impl IsTheoreticallyPossible for ConstraintMap {
    fn is_theoretically_possible(&self) -> bool {
        return !self
            .map
            .iter()
            .any(|(_, constraint)| !constraint.is_theoretically_possible());
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

impl Ord for ConstraintMap {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut current_order;
        let mut this_map = self.map.iter().map(|x| x.1).collect::<Vec<&Constraint>>();
        let mut other_map = other.map.iter().map(|x| x.1).collect::<Vec<&Constraint>>();

        this_map.sort();
        other_map.sort();

        for map_elements in this_map.iter().zip(other_map.iter()) {
            let (this_element, other_element) = map_elements;
            current_order = this_element.cmp(other_element);
            if current_order != Ordering::Equal {
                return current_order;
            }
        }
        this_map.len().cmp(&other_map.len())
    }
}

impl PartialOrd for ConstraintMap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
    use crate::constraint_management::ConstraintFactory;
    use crate::constraint_management::ConstraintMapFactory;

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
        assert!(constraint_map.compiles(HashMap::new()));
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
        assert!(constraint_map.compiles(id_value_map));
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
        assert!(!constraint_map.compiles(id_value_map));
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
        assert!(constraint_map.compiles(id_value_map));
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
        assert!(!constraint_map.compiles(id_value_map));
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
        assert!(constraint_map.compiles(id_value_map));
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
        assert!(!constraint_map.compiles(id_value_map));
    }

    #[test]
    fn combine_no_id_common() {
        let constraint_map_one = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(1, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        let constraint_map_two = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(3, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(4, vec![1, 2, 3]),
        ]);

        let constraint_map_three = constraint_map_one + constraint_map_two;

        assert!(constraint_map_three.map.contains_key(&1));
        assert!(constraint_map_three.map.contains_key(&2));
        assert!(constraint_map_three.map.contains_key(&3));
        assert!(constraint_map_three.map.contains_key(&4));
        assert!(!constraint_map_three.map.contains_key(&5));
    }

    #[test]
    fn combine_one_id_common() {
        let constraint_map_one = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(1, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        let constraint_map_two = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(2, vec![3, 4, 5]),
            ConstraintFactory::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);

        let constraint_map_three = constraint_map_one + constraint_map_two;

        assert!(constraint_map_three.map.contains_key(&1));
        assert!(constraint_map_three.map.contains_key(&2));
        assert!(constraint_map_three.map.contains_key(&3));
        assert!(!constraint_map_three
            .map
            .get(&2)
            .unwrap()
            .valid_values
            .contains(&1));
        assert!(!constraint_map_three
            .map
            .get(&2)
            .unwrap()
            .valid_values
            .contains(&2));
        assert!(constraint_map_three
            .map
            .get(&2)
            .unwrap()
            .valid_values
            .contains(&3));
        assert!(!constraint_map_three
            .map
            .get(&2)
            .unwrap()
            .valid_values
            .contains(&4));
        assert!(!constraint_map_three
            .map
            .get(&2)
            .unwrap()
            .valid_values
            .contains(&5));
        assert!(!constraint_map_three.map.contains_key(&4));
    }

    #[test]
    fn combine_all_id_common() {
        let constraint_map_one = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(1, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        let constraint_map_two = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(1, vec![2, 3, 4]),
            ConstraintFactory::new_many_item_constraint(2, vec![3, 4, 5]),
        ]);
        let constraint_map_three = constraint_map_one + constraint_map_two;

        assert!(constraint_map_three.map.contains_key(&1));
        assert!(!constraint_map_three
            .map
            .get(&1)
            .unwrap()
            .valid_values
            .contains(&1));
        assert!(constraint_map_three
            .map
            .get(&1)
            .unwrap()
            .valid_values
            .contains(&2));
        assert!(constraint_map_three
            .map
            .get(&1)
            .unwrap()
            .valid_values
            .contains(&3));
        assert!(!constraint_map_three
            .map
            .get(&1)
            .unwrap()
            .valid_values
            .contains(&4));
        assert!(constraint_map_three.map.contains_key(&2));
        assert!(!constraint_map_three
            .map
            .get(&2)
            .unwrap()
            .valid_values
            .contains(&1));
        assert!(!constraint_map_three
            .map
            .get(&2)
            .unwrap()
            .valid_values
            .contains(&2));
        assert!(constraint_map_three
            .map
            .get(&2)
            .unwrap()
            .valid_values
            .contains(&3));
        assert!(!constraint_map_three
            .map
            .get(&2)
            .unwrap()
            .valid_values
            .contains(&4));
        assert!(!constraint_map_three
            .map
            .get(&2)
            .unwrap()
            .valid_values
            .contains(&5));
        assert!(!constraint_map_three.map.contains_key(&4));
    }

    #[test]
    fn combine_no_impossable_options_common() {
        let constraint_map_one = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(1, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        let constraint_map_two = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(1, vec![2, 3, 4]),
            ConstraintFactory::new_many_item_constraint(2, vec![3, 4, 5]),
        ]);
        let constraint_map_three = constraint_map_one + constraint_map_two;

        assert!(constraint_map_three.map.contains_key(&1));
        assert!(!constraint_map_three
            .map
            .get(&1)
            .unwrap()
            .valid_values
            .contains(&1));
        assert!(constraint_map_three
            .map
            .get(&1)
            .unwrap()
            .valid_values
            .contains(&2));
        assert!(constraint_map_three
            .map
            .get(&1)
            .unwrap()
            .valid_values
            .contains(&3));
        assert!(!constraint_map_three
            .map
            .get(&1)
            .unwrap()
            .valid_values
            .contains(&4));
        assert!(constraint_map_three.map.contains_key(&2));
        assert!(!constraint_map_three
            .map
            .get(&2)
            .unwrap()
            .valid_values
            .contains(&1));
        assert!(!constraint_map_three
            .map
            .get(&2)
            .unwrap()
            .valid_values
            .contains(&2));
        assert!(constraint_map_three
            .map
            .get(&2)
            .unwrap()
            .valid_values
            .contains(&3));
        assert!(!constraint_map_three
            .map
            .get(&2)
            .unwrap()
            .valid_values
            .contains(&4));
        assert!(!constraint_map_three
            .map
            .get(&2)
            .unwrap()
            .valid_values
            .contains(&5));
        assert!(!constraint_map_three.map.contains_key(&4));
    }

    #[test]
    fn test_fmt() {
        let constraint1_123 = ConstraintFactory::new_many_item_constraint(1, vec![1, 2, 3]);
        let constraint2_123 = ConstraintFactory::new_many_item_constraint(2, vec![1, 2, 3]);
        let mut map: HashMap<u16, Constraint> = HashMap::new();
        map.insert(2, constraint2_123.clone());
        map.insert(1, constraint1_123.clone());

        let constraint_map =
            ConstraintMapFactory::new_constraint_map(vec![constraint1_123, constraint2_123]);
        assert_eq!(
            format!("{constraint_map:?}"),
            format!("ConstraintMap {{ map: {:?} }}", map)
        );
    }

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn test_clone() {
        let mut constraint_map_one = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(1, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        let constraint_map_two = constraint_map_one.clone();

        assert_eq!(constraint_map_one, constraint_map_two);
        constraint_map_one
            .map
            .insert(4, ConstraintFactory::new_empty_constraint(4));
        assert_ne!(constraint_map_one, constraint_map_two);
    }

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn test_clone_from() {
        let constraint_map_one = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(1, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        let mut constraint_map_two = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);
        constraint_map_two.clone_from(&constraint_map_one);
        assert_eq!(constraint_map_two.map.keys().count(), 2);
    }

    #[test]
    fn test_cmp_less() {
        let constraint_map_one = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(1, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        let constraint_map_two = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(2, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);
        let result = constraint_map_one.cmp(&constraint_map_two);
        assert_eq!(result, Ordering::Less);
    }

    #[test]
    fn test_cmp_less_longer_set() {
        let constraint_map_one = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(1, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        let constraint_map_two = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(1, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(2, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);
        let result = constraint_map_one.cmp(&constraint_map_two);
        assert_eq!(result, Ordering::Less);
    }

    #[test]
    fn test_cmp_greater() {
        let constraint_map_one = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(1, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        let constraint_map_two = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(2, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);
        let result = constraint_map_two.cmp(&constraint_map_one);
        assert_eq!(result, Ordering::Greater);
    }

    #[test]
    fn test_cmp_greater_longer_set() {
        let constraint_map_one = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(1, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        let constraint_map_two = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(1, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(2, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);
        let result = constraint_map_two.cmp(&constraint_map_one);
        assert_eq!(result, Ordering::Greater);
    }

    #[test]
    fn test_cmp_equal() {
        let constraint_map_one = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(1, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(2, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);
        let constraint_map_two = ConstraintMapFactory::new_constraint_map(vec![
            ConstraintFactory::new_many_item_constraint(1, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(2, vec![1, 2, 3]),
            ConstraintFactory::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);
        let result = constraint_map_two.cmp(&constraint_map_one);
        assert_eq!(result, Ordering::Equal);
    }
}
