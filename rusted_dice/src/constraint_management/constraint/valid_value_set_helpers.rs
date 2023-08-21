use crate::constraint_management::ValueTypeSet;

/// Combine two sets of valid values into one set of valid values.
/// the intersection of the two sets will be returned.
///
/// # Arguments
///
/// * `valid_values_one` - The first set of valid values.
/// * `valid_values_two` - The second set of valid values.
///
/// # Example
///
/// ```
/// # use crate::rusted_dice::constraint_management::combine_valid_value_sets;
/// let set_one = vec![1, 2, 3, 5, 6, 7].into_iter().collect();
/// let set_two = vec![2, 4, 7].into_iter().collect();
/// let expected_value = vec![2, 7].into_iter().collect();
///
/// let set_three = combine_valid_value_sets(&set_one, &set_two);
/// assert_eq!(
///    set_three
///      .difference(&expected_value)
///      .count(),
///    0
/// );
/// ```
pub fn combine_valid_value_sets(
    valid_values_one: &ValueTypeSet,
    valid_values_two: &ValueTypeSet,
) -> ValueTypeSet {
    valid_values_one
        .intersection(valid_values_two)
        .copied()
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::constraint_management::{combine_valid_value_sets, ValueTypeSet};

    #[test]
    fn combine_no_overlap() {
        let expected_value: ValueTypeSet = vec![].into_iter().collect();
        let set_one: ValueTypeSet = vec![1, 3, 5, 6].into_iter().collect();
        let set_two: ValueTypeSet = vec![4].into_iter().collect();

        let set_three = combine_valid_value_sets(&set_one, &set_two);
        assert_eq!(set_three.difference(&expected_value).count(), 0);
    }

    #[test]
    fn combine_part_overlap() {
        let expected_value: ValueTypeSet = vec![5, 6].into_iter().collect();
        let set_one: ValueTypeSet = vec![1, 3, 5, 6].into_iter().collect();
        let set_two: ValueTypeSet = vec![4, 5, 6].into_iter().collect();

        let set_three = combine_valid_value_sets(&set_one, &set_two);
        assert_eq!(set_three.difference(&expected_value).count(), 0);
    }

    #[test]
    fn combine_part_overlap_a_sub_set_of_b() {
        let expected_value: ValueTypeSet = vec![5, 6].into_iter().collect();
        let set_one: ValueTypeSet = vec![5, 6].into_iter().collect();
        let set_two: ValueTypeSet = vec![4, 5, 6, 7].into_iter().collect();

        let set_three = combine_valid_value_sets(&set_one, &set_two);
        assert_eq!(set_three.difference(&expected_value).count(), 0);
    }

    #[test]
    fn combine_part_overlap_b_sub_set_of_a() {
        let expected_value: ValueTypeSet = vec![5, 6].into_iter().collect();
        let set_one: ValueTypeSet = vec![4, 5, 6, 7].into_iter().collect();
        let set_two: ValueTypeSet = vec![5, 6].into_iter().collect();

        let set_three = combine_valid_value_sets(&set_one, &set_two);
        assert_eq!(set_three.difference(&expected_value).count(), 0);
    }

    #[test]
    fn combine_full_overlap() {
        let expected_value: ValueTypeSet = vec![4, 5, 6].into_iter().collect();
        let set_one: ValueTypeSet = vec![4, 5, 6].into_iter().collect();
        let set_two: ValueTypeSet = vec![4, 5, 6].into_iter().collect();

        let set_three = combine_valid_value_sets(&set_one, &set_two);
        assert_eq!(set_three.difference(&expected_value).count(), 0);
    }
}
