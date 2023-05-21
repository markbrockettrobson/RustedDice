use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::ops::Add;

use crate::constraint_management::ConstraintMap;

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
    use crate::constraint_management::{
        ConstraintFactory, ConstraintIdType, ConstraintMap, ConstraintMapFactory,
        ConstraintValueType,
    };

    fn test_map_has_key_valid_value(
        constraint_map: &ConstraintMap,
        id: ConstraintIdType,
        valid_value: ConstraintValueType,
    ) -> bool {
        constraint_map
            .map
            .get(&id)
            .unwrap()
            .valid_values
            .contains(&valid_value)
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
        assert!(!test_map_has_key_valid_value(&constraint_map_three, 2, 1));
        assert!(!test_map_has_key_valid_value(&constraint_map_three, 2, 2));
        assert!(test_map_has_key_valid_value(&constraint_map_three, 2, 3));
        assert!(!test_map_has_key_valid_value(&constraint_map_three, 2, 4));
        assert!(!test_map_has_key_valid_value(&constraint_map_three, 2, 5));
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
        assert!(!test_map_has_key_valid_value(&constraint_map_three, 1, 1));
        assert!(test_map_has_key_valid_value(&constraint_map_three, 1, 2));
        assert!(test_map_has_key_valid_value(&constraint_map_three, 1, 3));
        assert!(!test_map_has_key_valid_value(&constraint_map_three, 1, 4));

        assert!(constraint_map_three.map.contains_key(&2));
        assert!(!test_map_has_key_valid_value(&constraint_map_three, 2, 1));
        assert!(!test_map_has_key_valid_value(&constraint_map_three, 2, 2));
        assert!(test_map_has_key_valid_value(&constraint_map_three, 2, 3));
        assert!(!test_map_has_key_valid_value(&constraint_map_three, 2, 4));
        assert!(!test_map_has_key_valid_value(&constraint_map_three, 2, 5));
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
            ConstraintFactory::new_many_item_constraint(2, vec![4, 5, 6]),
        ]);
        let constraint_map_three = constraint_map_one + constraint_map_two;

        assert!(constraint_map_three.map.contains_key(&1));
        assert!(!test_map_has_key_valid_value(&constraint_map_three, 1, 1));
        assert!(test_map_has_key_valid_value(&constraint_map_three, 1, 2));
        assert!(test_map_has_key_valid_value(&constraint_map_three, 1, 3));
        assert!(!test_map_has_key_valid_value(&constraint_map_three, 1, 4));

        assert!(constraint_map_three.map.contains_key(&2));
        assert!(!test_map_has_key_valid_value(&constraint_map_three, 2, 1));
        assert!(!test_map_has_key_valid_value(&constraint_map_three, 2, 2));
        assert!(!test_map_has_key_valid_value(&constraint_map_three, 2, 3));
        assert!(!test_map_has_key_valid_value(&constraint_map_three, 2, 4));
        assert!(!test_map_has_key_valid_value(&constraint_map_three, 2, 5));
        assert!(!test_map_has_key_valid_value(&constraint_map_three, 2, 6));

        assert!(!constraint_map_three.map.contains_key(&4));
    }
}
