use std::ops::AddAssign;

use crate::constraint_management::ConstraintMap;

impl AddAssign for ConstraintMap {
    /// Implements the addition assignment operator for [ConstraintMap]. The intersection of maps are maintained
    /// the Constraint of matching keys are added together
    ///
    /// # Arguments
    ///
    /// * `self` - The first [ConstraintMap] operand.
    /// * `other` - The second [ConstraintMap] operand.
    ///
    /// # Returns
    ///
    /// The resulting [ConstraintMap] after the addition operation.
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::constraint_management::Constraint;
    /// # use crate::rusted_dice::constraint_management::ConstraintMap;
    /// let mut constraint_map_one = ConstraintMap::new_constraint_map(
    ///     vec![
    ///        Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
    ///        Constraint::new_many_item_constraint(2, vec![1, 2, 3])
    ///     ]
    /// );
    /// constraint_map_one += ConstraintMap::new_constraint_map(
    ///     vec![
    ///        Constraint::new_many_item_constraint(1, vec![3, 4, 5])
    ///     ]
    /// );
    /// let constraint_map_two = ConstraintMap::new_constraint_map(
    ///     vec![
    ///        Constraint::new_many_item_constraint(1, vec![3]),
    ///        Constraint::new_many_item_constraint(2, vec![1, 2, 3])
    ///     ]
    /// );
    /// assert_eq!(constraint_map_one, constraint_map_two);
    /// ```
    fn add_assign(&mut self, other: Self) {
        for (_, constraint) in other.map {
            *self += constraint;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        constraint_management::{Constraint, ConstraintIdType, ConstraintMap},
        ValueType,
    };

    fn has_key_valid_value(
        constraint_map: &ConstraintMap,
        id: ConstraintIdType,
        valid_value: ValueType,
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
        let mut constraint_map = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        constraint_map += ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(3, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(4, vec![1, 2, 3]),
        ]);

        assert_eq!(constraint_map.map.len(), 4);
        assert_eq!(constraint_map.map.get(&1).unwrap().valid_values.len(), 3);
        assert!(has_key_valid_value(&constraint_map, 1, 1));
        assert!(has_key_valid_value(&constraint_map, 1, 2));
        assert!(has_key_valid_value(&constraint_map, 1, 3));
        assert_eq!(constraint_map.map.get(&2).unwrap().valid_values.len(), 3);
        assert!(has_key_valid_value(&constraint_map, 2, 1));
        assert!(has_key_valid_value(&constraint_map, 2, 2));
        assert!(has_key_valid_value(&constraint_map, 2, 3));
        assert_eq!(constraint_map.map.get(&3).unwrap().valid_values.len(), 3);
        assert!(has_key_valid_value(&constraint_map, 3, 1));
        assert!(has_key_valid_value(&constraint_map, 3, 2));
        assert!(has_key_valid_value(&constraint_map, 3, 3));
        assert_eq!(constraint_map.map.get(&4).unwrap().valid_values.len(), 3);
        assert!(has_key_valid_value(&constraint_map, 4, 1));
        assert!(has_key_valid_value(&constraint_map, 4, 2));
        assert!(has_key_valid_value(&constraint_map, 4, 3));
    }

    #[test]
    fn combine_one_id_common() {
        let mut constraint_map = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        constraint_map += ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(2, vec![3, 4, 5]),
            Constraint::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);

        assert_eq!(constraint_map.map.len(), 3);
        assert_eq!(constraint_map.map.get(&1).unwrap().valid_values.len(), 3);
        assert!(has_key_valid_value(&constraint_map, 1, 1));
        assert!(has_key_valid_value(&constraint_map, 1, 2));
        assert!(has_key_valid_value(&constraint_map, 1, 3));
        assert_eq!(constraint_map.map.get(&2).unwrap().valid_values.len(), 1);
        assert!(has_key_valid_value(&constraint_map, 2, 3));
        assert_eq!(constraint_map.map.get(&3).unwrap().valid_values.len(), 3);
        assert!(has_key_valid_value(&constraint_map, 3, 1));
        assert!(has_key_valid_value(&constraint_map, 3, 2));
        assert!(has_key_valid_value(&constraint_map, 3, 3));
    }

    #[test]
    fn combine_all_id_common() {
        let mut constraint_map = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        constraint_map += ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![2, 3, 4]),
            Constraint::new_many_item_constraint(2, vec![3, 4, 5]),
        ]);

        assert_eq!(constraint_map.map.len(), 2);
        assert_eq!(constraint_map.map.get(&1).unwrap().valid_values.len(), 2);
        assert!(has_key_valid_value(&constraint_map, 1, 2));
        assert!(has_key_valid_value(&constraint_map, 1, 3));
        assert_eq!(constraint_map.map.get(&2).unwrap().valid_values.len(), 1);
        assert!(has_key_valid_value(&constraint_map, 2, 3));
    }

    #[test]
    fn combine_imposable_options() {
        let mut constraint_map = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        constraint_map += ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![2, 3, 4]),
            Constraint::new_many_item_constraint(2, vec![4, 5, 6]),
        ]);

        assert_eq!(constraint_map.map.len(), 2);
        assert_eq!(constraint_map.map.get(&1).unwrap().valid_values.len(), 2);
        assert!(has_key_valid_value(&constraint_map, 1, 2));
        assert!(has_key_valid_value(&constraint_map, 1, 3));
        assert_eq!(constraint_map.map.get(&2).unwrap().valid_values.len(), 0);
    }
}
