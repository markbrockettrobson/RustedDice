use std::ops::Add;

use crate::constraint_management::{Constraint, ConstraintMap};

use super::add_constraint_to_map;

impl Add<Constraint> for ConstraintMap {
    type Output = Self;
    /// Implements the addition operator for [ConstraintMap] + [Constraint].
    /// a Constraint of a maching key is added the existing Constraint
    ///
    /// # Arguments
    ///
    /// * `self` - The [ConstraintMap] operand.
    /// * `other` - The [Constraint] operand.
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
    ///        Constraint::new_many_item_constraint(1, vec![3]),
    ///        Constraint::new_many_item_constraint(2, vec![1, 2, 3])
    ///     ]
    /// );
    ///
    /// assert_eq!(
    ///     constraint_map_one + Constraint::new_many_item_constraint(1, vec![3, 4, 5]),
    ///     constraint_map_two
    /// );
    /// ```
    fn add(self, other: Constraint) -> Self {
        let mut new_map = self.map.clone();
        add_constraint_to_map(&mut new_map, other);
        ConstraintMap { map: new_map }
    }
}

impl Add<ConstraintMap> for Constraint {
    type Output = ConstraintMap;
    /// Implements the addition operator for [Constraint] + [ConstraintMap].
    /// a Constraint of a maching key is added the existing Constraint
    ///
    /// # Arguments
    ///
    /// * `self` - The [Constraint] operand.
    /// * `other` - The [ConstraintMap] operand.
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
    ///        Constraint::new_many_item_constraint(1, vec![3]),
    ///        Constraint::new_many_item_constraint(2, vec![1, 2, 3])
    ///     ]
    /// );
    ///
    /// assert_eq!(
    ///     Constraint::new_many_item_constraint(1, vec![3, 4, 5]) + constraint_map_one,
    ///     constraint_map_two
    /// );
    /// ```
    fn add(self, other: ConstraintMap) -> ConstraintMap {
        other + self
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
    fn combine_no_id_common_constraint_constraint_map() {
        let constraint_map_one =
            ConstraintMap::new_constraint_map(vec![Constraint::new_many_item_constraint(
                1,
                vec![1, 2, 3],
            )]);

        let constraint_map_two =
            constraint_map_one + Constraint::new_many_item_constraint(3, vec![1, 2, 3]);

        assert_eq!(constraint_map_two.map.len(), 2);
        assert_eq!(
            constraint_map_two.map.get(&1).unwrap().valid_values.len(),
            3
        );
        assert!(has_key_valid_value(&constraint_map_two, 1, 1));
        assert!(has_key_valid_value(&constraint_map_two, 1, 2));
        assert!(has_key_valid_value(&constraint_map_two, 1, 3));
        assert_eq!(
            constraint_map_two.map.get(&3).unwrap().valid_values.len(),
            3
        );
        assert!(has_key_valid_value(&constraint_map_two, 3, 1));
        assert!(has_key_valid_value(&constraint_map_two, 3, 2));
        assert!(has_key_valid_value(&constraint_map_two, 3, 3));
    }

    #[test]
    fn combine_id_common_constraint_constraint_map() {
        let constraint_map_one = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);

        let constraint_map_two =
            constraint_map_one + Constraint::new_many_item_constraint(2, vec![3, 4, 5]);

        assert_eq!(constraint_map_two.map.len(), 2);
        assert_eq!(
            constraint_map_two.map.get(&1).unwrap().valid_values.len(),
            3
        );
        assert!(has_key_valid_value(&constraint_map_two, 1, 1));
        assert!(has_key_valid_value(&constraint_map_two, 1, 2));
        assert!(has_key_valid_value(&constraint_map_two, 1, 3));
        assert_eq!(
            constraint_map_two.map.get(&2).unwrap().valid_values.len(),
            1
        );
        assert!(has_key_valid_value(&constraint_map_two, 2, 3));
    }

    #[test]
    fn combine_impossable_options_common_constraint_constraint_map() {
        let constraint_map_one = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        let constraint_map_two =
            constraint_map_one + Constraint::new_many_item_constraint(2, vec![4, 5, 6]);

        assert_eq!(constraint_map_two.map.len(), 2);
        assert_eq!(
            constraint_map_two.map.get(&1).unwrap().valid_values.len(),
            3
        );
        assert!(has_key_valid_value(&constraint_map_two, 1, 1));
        assert!(has_key_valid_value(&constraint_map_two, 1, 2));
        assert!(has_key_valid_value(&constraint_map_two, 1, 3));
        assert_eq!(
            constraint_map_two.map.get(&2).unwrap().valid_values.len(),
            0
        );
    }

    #[test]
    fn combine_no_id_common_constraint_map_constraint() {
        let constraint_map_one =
            ConstraintMap::new_constraint_map(vec![Constraint::new_many_item_constraint(
                1,
                vec![1, 2, 3],
            )]);

        let constraint_map_two =
            Constraint::new_many_item_constraint(3, vec![1, 2, 3]) + constraint_map_one;

        assert_eq!(constraint_map_two.map.len(), 2);
        assert_eq!(
            constraint_map_two.map.get(&1).unwrap().valid_values.len(),
            3
        );
        assert!(has_key_valid_value(&constraint_map_two, 1, 1));
        assert!(has_key_valid_value(&constraint_map_two, 1, 2));
        assert!(has_key_valid_value(&constraint_map_two, 1, 3));
        assert_eq!(
            constraint_map_two.map.get(&3).unwrap().valid_values.len(),
            3
        );
        assert!(has_key_valid_value(&constraint_map_two, 3, 1));
        assert!(has_key_valid_value(&constraint_map_two, 3, 2));
        assert!(has_key_valid_value(&constraint_map_two, 3, 3));
    }

    #[test]
    fn combine_id_common_constraint_map_constraint() {
        let constraint_map_one = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);

        let constraint_map_two =
            Constraint::new_many_item_constraint(2, vec![3, 4, 5]) + constraint_map_one;

        assert_eq!(constraint_map_two.map.len(), 2);
        assert_eq!(
            constraint_map_two.map.get(&1).unwrap().valid_values.len(),
            3
        );
        assert!(has_key_valid_value(&constraint_map_two, 1, 1));
        assert!(has_key_valid_value(&constraint_map_two, 1, 2));
        assert!(has_key_valid_value(&constraint_map_two, 1, 3));
        assert_eq!(
            constraint_map_two.map.get(&2).unwrap().valid_values.len(),
            1
        );
        assert!(has_key_valid_value(&constraint_map_two, 2, 3));
    }

    #[test]
    fn combine_impossable_options_common_constraint_map_constraint() {
        let constraint_map_one = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        let constraint_map_two =
            Constraint::new_many_item_constraint(2, vec![4, 5, 6]) + constraint_map_one;

        assert_eq!(constraint_map_two.map.len(), 2);
        assert_eq!(
            constraint_map_two.map.get(&1).unwrap().valid_values.len(),
            3
        );
        assert!(has_key_valid_value(&constraint_map_two, 1, 1));
        assert!(has_key_valid_value(&constraint_map_two, 1, 2));
        assert!(has_key_valid_value(&constraint_map_two, 1, 3));
        assert_eq!(
            constraint_map_two.map.get(&2).unwrap().valid_values.len(),
            0
        );
    }
}
