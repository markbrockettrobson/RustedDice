use crate::constraint_management::ConstraintIdToConstraintHashMap;

/// Represents a collection of Constraints.
///
/// Each ConstraintIdType can only have one Constraint
///
/// # Examples
/// #### A [ConstraintMap] without any Constraints
/// ```
/// # use crate::rusted_dice::constraint_management::ConstraintMap;
/// let constraint_map = ConstraintMap::new_empty_constraint_map();
/// ```
///
/// #### A [ConstraintMap] with one Constraint
/// ```
/// # use crate::rusted_dice::constraint_management::Constraint;
/// # use crate::rusted_dice::constraint_management::ConstraintMap;
/// let constraint = Constraint::new_many_item_constraint(1, vec![1, 2, 3]);
/// let constraint_map = ConstraintMap::new_single_constraint_constraint_map(constraint);
/// ```
///
/// #### A [ConstraintMap] with many Constraints,
/// Constraints with the same key will be combined
/// ```
/// # use crate::rusted_dice::constraint_management::Constraint;
/// # use crate::rusted_dice::constraint_management::ConstraintMap;
/// let constraint_1 = Constraint::new_many_item_constraint(1, vec![1, 2, 3]);
/// let constraint_2 = Constraint::new_many_item_constraint(2, vec![1, 2, 3]);
/// let constraint_3 = Constraint::new_many_item_constraint(3, vec![1, 2, 3]);
/// let constraint_4 = Constraint::new_many_item_constraint(4, vec![1, 2, 3]);
/// let constraint_map = ConstraintMap::new_constraint_map(
///     vec![
///        constraint_1,
///        constraint_2,
///        constraint_3,
///        constraint_4
///     ]
/// );
/// ```
///
/// #### Raw constructor
/// not recommended.
/// take care to correctly use the `Constraint.id` as key in the map.
/// ```
/// # use crate::rusted_dice::constraint_management::Constraint;
/// # use crate::rusted_dice::constraint_management::ConstraintMap;
/// # use crate::rusted_dice::constraint_management::ValueTypeSet;
/// # use crate::rusted_dice::constraint_management::ConstraintIdToConstraintHashMap;
/// # use std::collections::HashMap;
/// let constraint = Constraint::new_many_item_constraint(3, vec![1, 2, 3]);
/// let mut map: ConstraintIdToConstraintHashMap = HashMap::new();
/// map.insert(constraint.id, constraint);
/// let constraint_map = ConstraintMap { map };
/// ```
#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConstraintMap {
    pub map: ConstraintIdToConstraintHashMap,
}

#[cfg(test)]
mod tests {
    use crate::constraint_management::{Constraint, ConstraintMap};

    #[test]
    fn test_fmt() {
        let constraint_map = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1]),
            Constraint::new_many_item_constraint(2, vec![2]),
        ]);
        let different_orders = [
            "ConstraintMap { map: {1: Constraint { id: 1, valid_values: {1} }, 2: Constraint { id: 2, valid_values: {2} }} }",
            "ConstraintMap { map: {2: Constraint { id: 2, valid_values: {2} }, 1: Constraint { id: 1, valid_values: {1} }} }"
        ];
        assert!(different_orders.contains(&format!("{constraint_map:?}").as_str()));
    }

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn test_clone() {
        let mut constraint_map_one = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        let constraint_map_two = constraint_map_one.clone();

        assert_eq!(constraint_map_one, constraint_map_two);
        constraint_map_one
            .map
            .insert(4, Constraint::new_empty_constraint(4));
        assert_ne!(constraint_map_one, constraint_map_two);
    }

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn test_clone_from() {
        let constraint_map_one = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        let mut constraint_map_two =
            ConstraintMap::new_constraint_map(vec![Constraint::new_many_item_constraint(
                3,
                vec![1, 2, 3],
            )]);
        constraint_map_two.clone_from(&constraint_map_one);
        assert_eq!(constraint_map_two.map.keys().count(), 2);
    }

    #[test]
    fn test_eq_true() {
        let constraint_map_one = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        let constraint_map_two = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        assert!(constraint_map_one == constraint_map_two);
    }

    #[test]
    fn test_eq_true_empty() {
        let constraint_map_one = ConstraintMap::new_empty_constraint_map();
        let constraint_map_two = ConstraintMap::new_empty_constraint_map();
        assert!(constraint_map_one == constraint_map_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_missing_constraint_id() {
        let constraint_map_one = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        let constraint_map_two =
            ConstraintMap::new_constraint_map(vec![Constraint::new_many_item_constraint(
                2,
                vec![1, 2, 3],
            )]);
        assert!(constraint_map_one != constraint_map_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_extra_constraint_id() {
        let constraint_map_one = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        let constraint_map_two = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);
        assert!(constraint_map_one != constraint_map_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_no_overlapping_constraint_id() {
        let constraint_map_one = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        let constraint_map_two = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(10, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(20, vec![1, 2, 3]),
        ]);
        assert!(constraint_map_one != constraint_map_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_first_empty() {
        let constraint_map_one = ConstraintMap::new_empty_constraint_map();
        let constraint_map_two = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(10, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(20, vec![1, 2, 3]),
        ]);
        assert!(constraint_map_one != constraint_map_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_second_empty() {
        let constraint_map_one = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(10, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(20, vec![1, 2, 3]),
        ]);
        let constraint_map_two = ConstraintMap::new_empty_constraint_map();
        assert!(constraint_map_one != constraint_map_two);
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
    fn test_eq_false_no_different_valid_values() {
        let constraint_map_one = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2]),
        ]);
        let constraint_map_two = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3, 4]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        assert!(constraint_map_one != constraint_map_two);
    }
}
