use crate::constraint_management::Constraint;
use std::cmp::Ordering;

impl Ord for Constraint {
    /// Compare two `Constraint`s based on their ID and then valid values.
    ///
    /// first on id then on sorted valid values
    ///
    /// # Arguments
    ///
    /// * `self` - The first `Constraint` to compare.
    /// * `other` - The second `Constraint` to compare.
    ///
    /// # Returns
    ///
    /// An `Ordering` value indicating the relationship between the `Constraint`s.
    ///
    /// # Examples
    /// ```
    /// # use crate::rusted_dice::constraint_management::constraint::Constraint;
    /// let constraint_one = Constraint::new_many_item_constraint(1, vec![1, 2, 5]);
    /// let constraint_two = Constraint::new_many_item_constraint(4, vec![2, 5]);
    /// assert!(constraint_one < constraint_two);
    /// assert!(constraint_one.lt(&constraint_two));
    /// ```
    fn cmp(&self, other: &Self) -> Ordering {
        let mut this_set: Vec<_> = self.valid_values.iter().collect();
        let mut other_set: Vec<_> = other.valid_values.iter().collect();
        this_set.sort();
        other_set.sort();

        self.id.cmp(&other.id).then(this_set.cmp(&other_set))
    }
}

impl PartialOrd for Constraint {
    /// Compare two `Constraint`s partially based on their ID and valid values.
    ///
    /// Calls cmp
    ///
    /// # Arguments
    ///
    /// * `self` - The first `Constraint` to compare.
    /// * `other` - The second `Constraint` to compare.
    ///
    /// # Returns
    ///
    /// An `Option<Ordering>` value indicating the relationship between the `Constraints`,
    /// or `None` if the comparison cannot be determined.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmp_less() {
        let constraint_one = Constraint::new_many_item_constraint(1, vec![2, 4]);
        let constraint_two = Constraint::new_many_item_constraint(2, vec![2, 4, 6]);
        let result = constraint_one.cmp(&constraint_two);
        assert_eq!(result, Ordering::Less);
    }

    #[test]
    fn test_cmp_less_valid_values() {
        let constraint_one = Constraint::new_many_item_constraint(2, vec![2, 4, 5]);
        let constraint_two = Constraint::new_many_item_constraint(2, vec![2, 4, 6]);
        let result = constraint_one.cmp(&constraint_two);
        assert_eq!(result, Ordering::Less);
    }

    #[test]
    fn test_cmp_less_valid_values_uneven_length() {
        let constraint_one = Constraint::new_many_item_constraint(2, vec![2, 4]);
        let constraint_two = Constraint::new_many_item_constraint(2, vec![2, 4, 6]);
        let result = constraint_one.cmp(&constraint_two);
        assert_eq!(result, Ordering::Less);
    }

    #[test]
    fn test_cmp_less_valid_values_uneven_length_diferent_values() {
        let constraint_one = Constraint::new_many_item_constraint(2, vec![2, 4, 6]);
        let constraint_two = Constraint::new_many_item_constraint(2, vec![2, 7]);
        let result = constraint_one.cmp(&constraint_two);
        assert_eq!(result, Ordering::Less);
    }

    #[test]
    fn test_cmp_greater() {
        let constraint_one = Constraint::new_many_item_constraint(3, vec![2, 4, 6, 7]);
        let constraint_two = Constraint::new_many_item_constraint(2, vec![2, 4, 6, 8]);
        let result = constraint_one.cmp(&constraint_two);
        assert_eq!(result, Ordering::Greater);
    }

    #[test]
    fn test_cmp_greater_valid_values() {
        let constraint_one = Constraint::new_many_item_constraint(3, vec![2, 4, 7, 8]);
        let constraint_two = Constraint::new_many_item_constraint(3, vec![2, 4, 6, 8]);
        let result = constraint_one.cmp(&constraint_two);
        assert_eq!(result, Ordering::Greater);
    }

    #[test]
    fn test_cmp_greater_valid_values_uneven_length() {
        let constraint_one = Constraint::new_many_item_constraint(3, vec![2, 4, 6]);
        let constraint_two = Constraint::new_many_item_constraint(3, vec![2, 4]);
        let result = constraint_one.cmp(&constraint_two);
        assert_eq!(result, Ordering::Greater);
    }

    #[test]
    fn test_cmp_greater_valid_values_uneven_length_diferent_values() {
        let constraint_one = Constraint::new_many_item_constraint(3, vec![2, 4, 6]);
        let constraint_two = Constraint::new_many_item_constraint(3, vec![2, 3]);
        let result = constraint_one.cmp(&constraint_two);
        assert_eq!(result, Ordering::Greater);
    }

    #[test]
    fn test_cmp_equal() {
        let constraint_one = Constraint::new_many_item_constraint(321, vec![-3, 2, 4, 6, 89]);
        let constraint_two = Constraint::new_many_item_constraint(321, vec![-3, 2, 4, 6, 89]);
        let result = constraint_one.cmp(&constraint_two);
        assert_eq!(result, Ordering::Equal);
    }
}
