use crate::constraint_management::Constraint;
use std::cmp::Ordering;

impl Ord for Constraint {
    /// Compare two [Constraint]s based on their ID and then valid values.
    ///
    /// first on id then on sorted valid values
    ///
    /// # Arguments
    ///
    /// * `self` - The first [Constraint] to compare.
    /// * `other` - The second [Constraint] to compare.
    ///
    /// # Returns
    ///
    /// An [Ordering] value indicating the relationship between the [Constraint]s.
    ///
    /// # Examples
    /// ```
    /// # use crate::rusted_dice::constraint_management::Constraint;
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
    /// Compare two [Constraint]s partially based on their ID and valid values.
    ///
    /// Calls cmp
    ///
    /// # Arguments
    ///
    /// * `self` - The first [Constraint] to compare.
    /// * `other` - The second [Constraint] to compare.
    ///
    /// # Returns
    ///
    /// An `Option<Ordering>` value indicating the relationship between the [Constraint]s,
    /// or `None` if the comparison cannot be determined.
    ///
    /// # Examples
    /// ```
    /// # use crate::rusted_dice::constraint_management::Constraint;
    /// # use std::cmp::Ordering::{Equal, Greater, Less};
    /// let constraint_one = Constraint::new_many_item_constraint(1, vec![1, 2, 5]);
    /// let constraint_two = Constraint::new_many_item_constraint(4, vec![2, 5]);
    /// assert!(constraint_one.partial_cmp(&constraint_two) == Some(Less));
    /// assert!(constraint_one.partial_cmp(&constraint_one) == Some(Equal));
    /// assert!(constraint_two.partial_cmp(&constraint_one) == Some(Greater));
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering::{Equal, Greater, Less};

    #[test]
    fn test_cmp_less() {
        let constraint_one = Constraint::new_many_item_constraint(1, vec![2, 4]);
        let constraint_two = Constraint::new_many_item_constraint(2, vec![2, 4, 6]);
        assert_eq!(constraint_one.cmp(&constraint_two), Less);
        assert_eq!(constraint_one.partial_cmp(&constraint_two), Some(Less));
    }

    #[test]
    fn test_cmp_less_valid_values() {
        let constraint_one = Constraint::new_many_item_constraint(2, vec![2, 4, 5]);
        let constraint_two = Constraint::new_many_item_constraint(2, vec![2, 4, 6]);
        assert_eq!(constraint_one.cmp(&constraint_two), Less);
        assert_eq!(constraint_one.partial_cmp(&constraint_two), Some(Less));
    }

    #[test]
    fn test_cmp_less_valid_values_uneven_length() {
        let constraint_one = Constraint::new_many_item_constraint(2, vec![2, 4]);
        let constraint_two = Constraint::new_many_item_constraint(2, vec![2, 4, 6]);
        assert_eq!(constraint_one.cmp(&constraint_two), Less);
        assert_eq!(constraint_one.partial_cmp(&constraint_two), Some(Less));
    }

    #[test]
    fn test_cmp_less_valid_values_uneven_length_different_values() {
        let constraint_one = Constraint::new_many_item_constraint(2, vec![2, 4, 6]);
        let constraint_two = Constraint::new_many_item_constraint(2, vec![2, 7]);
        assert_eq!(constraint_one.cmp(&constraint_two), Less);
        assert_eq!(constraint_one.partial_cmp(&constraint_two), Some(Less));
    }

    #[test]
    fn test_cmp_greater() {
        let constraint_one = Constraint::new_many_item_constraint(3, vec![2, 4, 6, 7]);
        let constraint_two = Constraint::new_many_item_constraint(2, vec![2, 4, 6, 8]);
        assert_eq!(constraint_one.cmp(&constraint_two), Greater);
        assert_eq!(constraint_one.partial_cmp(&constraint_two), Some(Greater));
    }

    #[test]
    fn test_cmp_greater_valid_values() {
        let constraint_one = Constraint::new_many_item_constraint(3, vec![2, 4, 7, 8]);
        let constraint_two = Constraint::new_many_item_constraint(3, vec![2, 4, 6, 8]);
        assert_eq!(constraint_one.cmp(&constraint_two), Greater);
        assert_eq!(constraint_one.partial_cmp(&constraint_two), Some(Greater));
    }

    #[test]
    fn test_cmp_greater_valid_values_uneven_length() {
        let constraint_one = Constraint::new_many_item_constraint(3, vec![2, 4, 6]);
        let constraint_two = Constraint::new_many_item_constraint(3, vec![2, 4]);
        assert_eq!(constraint_one.cmp(&constraint_two), Greater);
        assert_eq!(constraint_one.partial_cmp(&constraint_two), Some(Greater));
    }

    #[test]
    fn test_cmp_greater_valid_values_uneven_length_different_values() {
        let constraint_one = Constraint::new_many_item_constraint(3, vec![2, 4, 6]);
        let constraint_two = Constraint::new_many_item_constraint(3, vec![2, 3]);
        assert_eq!(constraint_one.cmp(&constraint_two), Greater);
        assert_eq!(constraint_one.partial_cmp(&constraint_two), Some(Greater));
    }

    #[test]
    fn test_cmp_equal() {
        let constraint_one = Constraint::new_many_item_constraint(321, vec![-3, 2, 4, 6, 89]);
        let constraint_two = Constraint::new_many_item_constraint(321, vec![-3, 2, 4, 6, 89]);
        assert_eq!(constraint_one.cmp(&constraint_two), Equal);
        assert_eq!(constraint_one.partial_cmp(&constraint_two), Some(Equal));
    }
}
