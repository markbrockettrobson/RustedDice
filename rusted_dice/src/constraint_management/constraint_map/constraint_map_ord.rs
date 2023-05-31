use std::cmp::Ordering;

use crate::constraint_management::{Constraint, ConstraintMap};

impl Ord for ConstraintMap {
    /// Compare two constraints_map based on their constraints in order.
    ///
    /// # Arguments
    ///
    /// * `self` - The first constraint to compare.
    /// * `other` - The second constraint to compare.
    ///
    /// # Returns
    ///
    /// An `Ordering` value indicating the relationship between the constraints.
    ///
    /// # Examples
    /// ```
    /// # use crate::rusted_dice::constraint_management::constraint::Constraint;
    /// # use crate::rusted_dice::constraint_management::constraint_map::ConstraintMap;
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
    /// assert!(constraint_map_one < constraint_map_two);
    /// assert!(constraint_map_one.lt(&constraint_map_two));
    /// ```
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
    /// Compare two constraints maps partially based on their constraints in order.
    ///
    /// Calls cmp
    ///
    /// # Arguments
    ///
    /// * `self` - The first constraint to compare.
    /// * `other` - The second constraint to compare.
    ///
    /// # Returns
    ///
    /// An `Option<Ordering>` value indicating the relationship between the constraintmaps,
    /// or `None` if the comparison cannot be determined.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::constraint_management::{Constraint, ConstraintMap};

    use super::*;

    #[test]
    fn test_cmp_less() {
        let constraint_map_one = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        let constraint_map_two = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);
        let result = constraint_map_one.cmp(&constraint_map_two);
        assert_eq!(result, Ordering::Less);
    }

    #[test]
    fn test_cmp_less_longer_set() {
        let constraint_map_one = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        let constraint_map_two = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);
        let result = constraint_map_one.cmp(&constraint_map_two);
        assert_eq!(result, Ordering::Less);
    }

    #[test]
    fn test_cmp_greater() {
        let constraint_map_one = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        let constraint_map_two = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);
        let result = constraint_map_two.cmp(&constraint_map_one);
        assert_eq!(result, Ordering::Greater);
    }

    #[test]
    fn test_cmp_greater_longer_set() {
        let constraint_map_one = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
        ]);
        let constraint_map_two = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);
        let result = constraint_map_two.cmp(&constraint_map_one);
        assert_eq!(result, Ordering::Greater);
    }

    #[test]
    fn test_cmp_equal() {
        let constraint_map_one = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);
        let constraint_map_two = ConstraintMap::new_constraint_map(vec![
            Constraint::new_many_item_constraint(1, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(2, vec![1, 2, 3]),
            Constraint::new_many_item_constraint(3, vec![1, 2, 3]),
        ]);
        let result = constraint_map_two.cmp(&constraint_map_one);
        assert_eq!(result, Ordering::Equal);
    }
}
