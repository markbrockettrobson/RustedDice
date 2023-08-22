use std::collections::HashMap;

use crate::constraint_management::{Constraint, ConstraintIdToConstraintHashMap, ConstraintMap};

use super::add_constraint_to_map;

#[allow(dead_code)]
impl ConstraintMap {
    /// Creates a new empty [ConstraintMap].
    ///
    /// # Returns
    ///
    /// The new empty [ConstraintMap].
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::constraint_management::ConstraintMap;
    /// let constraint_map = ConstraintMap::new_empty_constraint_map();
    /// ```
    pub fn new_empty_constraint_map() -> ConstraintMap {
        let map: ConstraintIdToConstraintHashMap = HashMap::new();
        ConstraintMap { map }
    }

    /// Creates a new [ConstraintMap] with the given [Constraint].
    ///
    /// # Arguments
    ///
    /// * `constraint`: a [Constraint] to be the only value in the map.
    ///
    /// # Returns
    ///
    /// The new [ConstraintMap] the [Constraint].
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::constraint_management::ConstraintMap;
    /// # use crate::rusted_dice::constraint_management::Constraint;
    /// let constraint = Constraint::new_single_valid_value_constraint(2, 3);
    /// let constraint_map = ConstraintMap::new_single_constraint_constraint_map(constraint);
    /// ```
    pub fn new_single_constraint_constraint_map(constraint: Constraint) -> ConstraintMap {
        let mut map: ConstraintIdToConstraintHashMap = HashMap::new();
        add_constraint_to_map(&mut map, constraint);
        ConstraintMap { map }
    }

    /// Creates a new [ConstraintMap] from an iterator of [Constraint]s.
    ///
    /// This function takes an iterator of [Constraint] items and constructs a [ConstraintMap]
    /// by consolidating the [Constraint]s based on their IDs. If multiple [Constraint]s with the same
    /// ID are found, they will be merged using the '+' operator of the [Constraint] type.
    ///
    /// # Arguments
    ///
    /// * 'constraints': An iterator of [Constraint] items.
    ///
    /// # Returns
    ///
    /// The new [ConstraintMap] containing the merged [Constraint]s.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::collections::HashSet;
    /// # use crate::rusted_dice::constraint_management::ConstraintMap;
    /// # use crate::rusted_dice::constraint_management::Constraint;
    ///
    /// let constraints = vec![
    ///     Constraint::new_single_valid_value_constraint(1, 1),
    ///     Constraint::new_single_valid_value_constraint(2, 2),
    ///     Constraint::new_single_valid_value_constraint(3, 3),
    ///     Constraint::new_single_valid_value_constraint(3, 4),
    /// ];
    ///
    /// let constraint_map = ConstraintMap::new_constraint_map(constraints);
    /// let unique_constraint_ids: HashSet<u16> = constraint_map.map.keys().cloned().collect();
    /// assert_eq!(unique_constraint_ids, vec![1, 2, 3].into_iter().collect());
    /// ```
    pub fn new_constraint_map(constraints: impl IntoIterator<Item = Constraint>) -> ConstraintMap {
        let mut map: ConstraintIdToConstraintHashMap = HashMap::new();
        for constraint in constraints {
            add_constraint_to_map(&mut map, constraint);
        }
        ConstraintMap { map }
    }
}

#[cfg(test)]
mod tests {
    use crate::constraint_management::{Constraint, ConstraintIdToConstraintHashMap};

    use super::*;
    use std::vec::IntoIter;

    #[test]
    fn test_new_empty_constraint_map() {
        let map: ConstraintIdToConstraintHashMap = HashMap::new();
        let constraint_map = ConstraintMap { map };
        assert_eq!(constraint_map, ConstraintMap::new_empty_constraint_map());
    }

    #[test]
    fn test_new_constraint_map() {
        let constraint3_123 = Constraint::new_many_item_constraint(3, vec![1, 2, 3]);
        let mut map: ConstraintIdToConstraintHashMap = HashMap::new();
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

        let map: ConstraintIdToConstraintHashMap = HashMap::new();
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

        let mut map: ConstraintIdToConstraintHashMap = HashMap::new();
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

        let mut map: ConstraintIdToConstraintHashMap = HashMap::new();
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

        let mut map: ConstraintIdToConstraintHashMap = HashMap::new();
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
