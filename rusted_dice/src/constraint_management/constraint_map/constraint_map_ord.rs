use std::cmp::Ordering;

use crate::constraint_management::{Constraint, ConstraintMap};

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

#[cfg(test)]
mod tests {
    use crate::constraint_management::{ConstraintFactory, ConstraintMapFactory};

    use super::*;

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
