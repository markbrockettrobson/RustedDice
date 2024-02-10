use std::ops::Add;

use crate::constraint_management::ConstraintMap;

use super::add_constraint_to_map;

impl Add for ConstraintMap {
    type Output = Self;
    /// Implements the addition operator for [ConstraintMap]. The intersection of maps are maintained
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
    /// let constraint_map_one = ConstraintMap::new_constraint_map(
    ///     vec![
    ///        Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
    ///        Constraint::new_many_item_constraint(2, vec![1, 2, 3])
    ///     ]
    /// );
    /// let constraint_map_two = ConstraintMap::new_constraint_map(
    ///     vec![
    ///        Constraint::new_many_item_constraint(1, vec![3, 4, 5])
    ///     ]
    /// );
    /// let constraint_map_three = ConstraintMap::new_constraint_map(
    ///     vec![
    ///        Constraint::new_many_item_constraint(1, vec![3]),
    ///        Constraint::new_many_item_constraint(2, vec![1, 2, 3])
    ///     ]
    /// );
    ///
    /// assert_eq!(constraint_map_one + constraint_map_two, constraint_map_three);
    /// ```

    fn add(self, other: Self) -> Self {
        let mut new_map = self.map.clone();

        for (_, constraint) in other.map {
            add_constraint_to_map(&mut new_map, constraint);
        }
        ConstraintMap { map: new_map }
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
        let constraint_map_one = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        let constraint_map_two = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(3, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(4, vec![1, 2, 3]),
        ]);

        let constraint_map_three = constraint_map_one + constraint_map_two;

        assert_eq!(constraint_map_three.map.len(), 4);
        assert_eq!(
            constraint_map_three.map.get(&1).unwrap().valid_values.len(),
            3
        );
        assert!(has_key_valid_value(&constraint_map_three, 1, 1));
        assert!(has_key_valid_value(&constraint_map_three, 1, 2));
        assert!(has_key_valid_value(&constraint_map_three, 1, 3));
        assert_eq!(
            constraint_map_three.map.get(&2).unwrap().valid_values.len(),
            3
        );
        assert!(has_key_valid_value(&constraint_map_three, 2, 1));
        assert!(has_key_valid_value(&constraint_map_three, 2, 2));
        assert!(has_key_valid_value(&constraint_map_three, 2, 3));
        assert_eq!(
            constraint_map_three.map.get(&3).unwrap().valid_values.len(),
            3
        );
        assert!(has_key_valid_value(&constraint_map_three, 3, 1));
        assert!(has_key_valid_value(&constraint_map_three, 3, 2));
        assert!(has_key_valid_value(&constraint_map_three, 3, 3));
        assert_eq!(
            constraint_map_three.map.get(&4).unwrap().valid_values.len(),
            3
        );
        assert!(has_key_valid_value(&constraint_map_three, 4, 1));
        assert!(has_key_valid_value(&constraint_map_three, 4, 2));
        assert!(has_key_valid_value(&constraint_map_three, 4, 3));
    }

    #[test]
    fn combine_one_id_common() {
        let constraint_map_one = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        let constraint_map_two = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(2, vec![3, 4, 5]),
            Constraint::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);

        let constraint_map_three = constraint_map_one + constraint_map_two;

        assert_eq!(constraint_map_three.map.len(), 3);
        assert_eq!(
            constraint_map_three.map.get(&1).unwrap().valid_values.len(),
            3
        );
        assert!(has_key_valid_value(&constraint_map_three, 1, 1));
        assert!(has_key_valid_value(&constraint_map_three, 1, 2));
        assert!(has_key_valid_value(&constraint_map_three, 1, 3));
        assert_eq!(
            constraint_map_three.map.get(&2).unwrap().valid_values.len(),
            1
        );
        assert!(has_key_valid_value(&constraint_map_three, 2, 3));
        assert_eq!(
            constraint_map_three.map.get(&3).unwrap().valid_values.len(),
            3
        );
        assert!(has_key_valid_value(&constraint_map_three, 3, 1));
        assert!(has_key_valid_value(&constraint_map_three, 3, 2));
        assert!(has_key_valid_value(&constraint_map_three, 3, 3));
    }

    #[test]
    fn combine_all_id_common() {
        let constraint_map_one = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        let constraint_map_two = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![2, 3, 4]),
            Constraint::new_many_item_constraint(2, vec![3, 4, 5]),
        ]);
        let constraint_map_three = constraint_map_one + constraint_map_two;

        assert_eq!(constraint_map_three.map.len(), 2);
        assert_eq!(
            constraint_map_three.map.get(&1).unwrap().valid_values.len(),
            2
        );
        assert!(has_key_valid_value(&constraint_map_three, 1, 2));
        assert!(has_key_valid_value(&constraint_map_three, 1, 3));
        assert_eq!(
            constraint_map_three.map.get(&2).unwrap().valid_values.len(),
            1
        );
        assert!(has_key_valid_value(&constraint_map_three, 2, 3));
    }

    #[test]
    fn combine_imposable_options_common() {
        let constraint_map_one = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        let constraint_map_two = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![2, 3, 4]),
            Constraint::new_many_item_constraint(2, vec![4, 5, 6]),
        ]);
        let constraint_map_three = constraint_map_one + constraint_map_two;

        assert_eq!(constraint_map_three.map.len(), 2);
        assert_eq!(
            constraint_map_three.map.get(&1).unwrap().valid_values.len(),
            2
        );
        assert!(has_key_valid_value(&constraint_map_three, 1, 2));
        assert!(has_key_valid_value(&constraint_map_three, 1, 3));
        assert_eq!(
            constraint_map_three.map.get(&2).unwrap().valid_values.len(),
            0
        );
    }
}
