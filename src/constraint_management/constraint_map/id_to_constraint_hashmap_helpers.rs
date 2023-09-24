use crate::constraint_management::{Constraint, ConstraintIdToConstraintHashMap};
use std::collections::hash_map::Entry::{Occupied, Vacant};

/// a helper function to add a [Constraint] to a [ConstraintIdToConstraintHashMap]
/// if the constraint already exists in the map, the constraint will be combined with the existing constraint
///
/// # Arguments
///
/// * `constraint_hash_map` - the [ConstraintIdToConstraintHashMap] to add the constraint to
/// * `constraint` - the [Constraint] to add to the map
///
/// # Examples
///
/// ```
/// # use crate::rusted_dice::constraint_management::add_constraint_to_map;
/// # use crate::rusted_dice::constraint_management::{ConstraintIdToConstraintHashMap, Constraint};
/// # use std::collections::HashMap;
/// let mut map: ConstraintIdToConstraintHashMap = HashMap::new();
///
/// let constraint1_123 = Constraint::new_many_item_constraint(1, vec![1, 2, 3]);
/// let constraint2_123 = Constraint::new_many_item_constraint(2, vec![1, 2, 3]);
/// let constraint2_345 = Constraint::new_many_item_constraint(2, vec![3, 4, 5]);
/// add_constraint_to_map(&mut map, constraint1_123);
/// add_constraint_to_map(&mut map, constraint2_123);
/// add_constraint_to_map(&mut map, constraint2_345);
///
/// assert_eq!(
///    map.get(&1).unwrap().valid_values,
///    vec![1, 2, 3].into_iter().collect()
/// );
/// assert_eq!(
///    map.get(&2).unwrap().valid_values,
///    vec![3].into_iter().collect()
/// );
/// ```
pub fn add_constraint_to_map(
    constraint_hash_map: &mut ConstraintIdToConstraintHashMap,
    constraint: Constraint,
) {
    match constraint_hash_map.entry(constraint.id) {
        Occupied(mut entry) => {
            *entry.get_mut() += constraint;
        }
        Vacant(entry) => {
            entry.insert(constraint);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        constraint_management::{
            add_constraint_to_map, Constraint, ConstraintIdToConstraintHashMap, ConstraintIdType,
        },
        ValueType,
    };

    fn has_key_valid_value(
        constraint_map: &ConstraintIdToConstraintHashMap,
        id: ConstraintIdType,
        valid_value: ValueType,
    ) -> bool {
        constraint_map
            .get(&id)
            .unwrap()
            .valid_values
            .contains(&valid_value)
    }

    #[test]
    fn add_constraint_to_map_add_to_empty() {
        let mut map: ConstraintIdToConstraintHashMap = HashMap::new();

        let constraint1_123 = Constraint::new_many_item_constraint(1, vec![1, 2, 3]);
        add_constraint_to_map(&mut map, constraint1_123.clone());

        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&1).unwrap().valid_values.len(), 3);
        assert!(has_key_valid_value(&map, 1, 1));
        assert!(has_key_valid_value(&map, 1, 2));
        assert!(has_key_valid_value(&map, 1, 3));
    }

    #[test]
    fn add_constraint_to_map_add_no_overlap() {
        let mut map: ConstraintIdToConstraintHashMap = HashMap::new();

        let constraint1_123 = Constraint::new_many_item_constraint(1, vec![1, 2, 3]);
        map.insert(1, constraint1_123.clone());

        let constraint2_123 = Constraint::new_many_item_constraint(2, vec![1, 2, 3]);
        add_constraint_to_map(&mut map, constraint2_123.clone());

        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&1).unwrap().valid_values.len(), 3);
        assert!(has_key_valid_value(&map, 1, 1));
        assert!(has_key_valid_value(&map, 1, 2));
        assert!(has_key_valid_value(&map, 1, 3));

        assert_eq!(map.get(&2).unwrap().valid_values.len(), 3);
        assert!(has_key_valid_value(&map, 2, 1));
        assert!(has_key_valid_value(&map, 2, 2));
        assert!(has_key_valid_value(&map, 2, 3));
    }

    #[test]
    fn add_constraint_to_map_add_overlap() {
        let mut map: ConstraintIdToConstraintHashMap = HashMap::new();

        let constraint4_123 = Constraint::new_many_item_constraint(4, vec![1, 2, 3]);
        map.insert(4, constraint4_123.clone());

        let constraint4_345 = Constraint::new_many_item_constraint(4, vec![3, 4, 5]);
        add_constraint_to_map(&mut map, constraint4_345.clone());

        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&4).unwrap().valid_values.len(), 1);
        assert!(!has_key_valid_value(&map, 4, 1));
        assert!(!has_key_valid_value(&map, 4, 2));
        assert!(has_key_valid_value(&map, 4, 3));
        assert!(!has_key_valid_value(&map, 4, 4));
        assert!(!has_key_valid_value(&map, 4, 5));
    }
}
