use crate::{constraint_management::ConstraintMap, ValueType};

/// Represents a [ProbabilityOutcome].
///
/// Each [ProbabilityOutcome] has a [ValueType] ('value') and a [ConstraintMap] (`constraint_map`).
/// This class represents a single outcome of a ProbabilityDistribution.
/// The [ConstraintMap] holds information on Constraints that may prevent one or more [ProbabilityOutcome]s from being combined
///
/// # Examples
/// #### A [ProbabilityOutcome] with no Constraints
/// ```
/// # use crate::rusted_dice::probability::ProbabilityOutcome;
/// let value = 123;
/// let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(value);
/// ```
///
/// #### A [ProbabilityOutcome] with a given [ConstraintMap]
/// ```
/// # use crate::rusted_dice::probability::ProbabilityOutcome;
/// # use crate::rusted_dice::constraint_management::ConstraintMap;
/// let value = 123;
/// let constraint_map = ConstraintMap::new_empty_constraint_map();
/// let probability_outcome = ProbabilityOutcome::new_with_constraint_map(value, constraint_map);
/// ```
///
/// #### builds a new [ConstraintMap] from many Constraints,
/// Constraints with the same key will be combined see ConstraintMap::new_constraint_map
/// ```
/// # use crate::rusted_dice::probability::ProbabilityOutcome;
/// # use crate::rusted_dice::constraint_management::Constraint;
/// let value = 123;
/// let constraint_1 = Constraint::new_many_item_constraint(1, vec![1, 2, 3]);
/// let constraint_2 = Constraint::new_many_item_constraint(2, vec![1, 2, 3]);
/// let probability_outcome = ProbabilityOutcome::new_with_constraints(
///     value,
///     vec![constraint_1, constraint_2]
/// );
/// ```
#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct ProbabilityOutcome {
    pub value: ValueType,
    pub constraint_map: ConstraintMap,
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering::{Equal, Greater, Less};

    use crate::constraint_management::{Constraint, ConstraintIdType, ConstraintMap};
    use crate::probability::ProbabilityOutcome;
    use crate::{SmallValueType, UnsignedSmallValueType, ValueType};

    use proptest::prelude::*;

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

    fn key_valid_value_len(constraint_map: &ConstraintMap, id: ConstraintIdType) -> usize {
        constraint_map.map.get(&id).unwrap().valid_values.len()
    }

    proptest! {
        #[test]
        fn test_value_set(test_value: ValueType) {
            let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(test_value);
            assert!(probability_outcome.value == test_value);
        }

        #[test]
        fn test_constraint_map_set(test_value: ConstraintIdType) {
            let probability_outcome = ProbabilityOutcome::new_with_constraints(
                123,
                vec![
                    Constraint::new_many_item_constraint(test_value, vec![1,2,3])
            ]);
            assert_eq!(probability_outcome.constraint_map.map.len(), 1);
            assert_eq!(
                key_valid_value_len(&probability_outcome.constraint_map, test_value), 3);
            assert!(has_key_valid_value(&probability_outcome.constraint_map, test_value, 1));
            assert!(has_key_valid_value(&probability_outcome.constraint_map, test_value, 2));
            assert!(has_key_valid_value(&probability_outcome.constraint_map, test_value, 3));
        }

        #[test]
        fn test_eq_true(test_value: ConstraintIdType) {
            let probability_outcome_one = ProbabilityOutcome::new_with_constraints(
                123,
                vec![
                    Constraint::new_many_item_constraint(test_value, vec![1,2,3])
                ]);

            let probability_outcome_two = ProbabilityOutcome::new_with_constraints(
                123,
                vec![
                    Constraint::new_many_item_constraint(test_value, vec![1,2,3])
                ]);
            assert!(probability_outcome_one == probability_outcome_two);
        }

        #[test]
        #[allow(clippy::nonminimal_bool)]
        fn test_eq_false_value(test_value: ValueType, other_test_value: ValueType) {
            prop_assume!(test_value != other_test_value);
            let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(test_value);
            let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(other_test_value);
            assert!(!(probability_outcome_one == probability_outcome_two));
        }

        #[test]
        #[allow(clippy::nonminimal_bool)]
        fn test_eq_false_constraints(test_value: ValueType) {
            let probability_outcome_one = ProbabilityOutcome::new_with_constraints(
                test_value,
                vec![
                    Constraint::new_many_item_constraint(123, vec![1,2,3])
                ]);

            let probability_outcome_two = ProbabilityOutcome::new_with_constraints(
                test_value,
                vec![
                    Constraint::new_many_item_constraint(12, vec![1,2])
                ]);
            assert!(!(probability_outcome_one == probability_outcome_two));
        }

        #[test]
        #[allow(clippy::nonminimal_bool)]
        fn test_ne_false(test_value: ConstraintIdType) {
            let probability_outcome_one = ProbabilityOutcome::new_with_constraints(
                123,
                vec![
                    Constraint::new_many_item_constraint(test_value, vec![1,2,3])
                ]);

            let probability_outcome_two = ProbabilityOutcome::new_with_constraints(
                123,
                vec![
                    Constraint::new_many_item_constraint(test_value, vec![1,2,3])
                ]);
            assert!(!(probability_outcome_one != probability_outcome_two));
        }

        #[test]
        fn test_ne_true_value(test_value: ValueType, other_test_value: ValueType) {
            prop_assume!(test_value != other_test_value);
            let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(test_value);
            let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(other_test_value);
            assert!(probability_outcome_one != probability_outcome_two);
        }

        #[test]
        fn test_ne_true_constraints(test_value: ValueType) {
            let probability_outcome_one = ProbabilityOutcome::new_with_constraints(
                test_value,
                vec![
                    Constraint::new_many_item_constraint(123, vec![1,2,3])
                ]);
            let probability_outcome_two = ProbabilityOutcome::new_with_constraints(
                test_value,
                vec![
                    Constraint::new_many_item_constraint(12, vec![1,2])
                ]);
            assert!(probability_outcome_one != probability_outcome_two);
        }

        #[test]
        fn test_gt_true(base_value: SmallValueType, delta: UnsignedSmallValueType) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(base_value.into());
            let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(ValueType::from(base_value) + ValueType::from(delta));
            assert!(probability_outcome_two > probability_outcome_one);
        }

        #[test]
        #[allow(clippy::nonminimal_bool)]
        fn test_gt_false(base_value: SmallValueType, delta: UnsignedSmallValueType) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(base_value.into());
            let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(ValueType::from(base_value) + ValueType::from(delta));
            assert!(!(probability_outcome_one > probability_outcome_two));
        }

        #[test]
        #[allow(clippy::nonminimal_bool)]
        fn test_gt_same(base_value: SmallValueType) {
            let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(base_value.into());
            let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(base_value.into());
            assert!(!(probability_outcome_two > probability_outcome_one));
        }

        #[test]
        fn test_lt_true(base_value: SmallValueType, delta: UnsignedSmallValueType) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(base_value.into());
            let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(ValueType::from(base_value) + ValueType::from(delta));
            assert!(probability_outcome_one < probability_outcome_two);
        }

        #[test]
        #[allow(clippy::nonminimal_bool)]
        fn test_lt_false(base_value: SmallValueType, delta: UnsignedSmallValueType) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(base_value.into());
            let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(ValueType::from(base_value) + ValueType::from(delta));
            assert!(!(probability_outcome_two < probability_outcome_one));
        }

        #[test]
        #[allow(clippy::nonminimal_bool)]
        fn test_lt_same(base_value: SmallValueType) {
            let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(base_value.into());
            let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(base_value.into());
            assert!(!(probability_outcome_two < probability_outcome_one));
        }

        #[test]
        fn test_ge_true(base_value: SmallValueType, delta: UnsignedSmallValueType) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(base_value.into());
            let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(ValueType::from(base_value) + ValueType::from(delta));
            assert!(probability_outcome_two >= probability_outcome_one);
        }

        #[test]
        #[allow(clippy::nonminimal_bool)]
        fn test_ge_false(base_value: SmallValueType, delta: UnsignedSmallValueType) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(base_value.into());
            let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(ValueType::from(base_value) + ValueType::from(delta));
            assert!(!(probability_outcome_one >= probability_outcome_two));
        }

        #[test]
        fn test_ge_same(base_value: SmallValueType) {
            let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(base_value.into());
            let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(base_value.into());
            assert!(probability_outcome_two >= probability_outcome_one);
        }

        #[test]
        fn test_le_true(base_value: SmallValueType, delta: UnsignedSmallValueType) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(base_value.into());
            let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(ValueType::from(base_value) + ValueType::from(delta));
            assert!(probability_outcome_one <= probability_outcome_two);
        }

        #[test]
        #[allow(clippy::nonminimal_bool)]
        fn test_le_false(base_value: SmallValueType, delta: UnsignedSmallValueType) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(base_value.into());
            let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(ValueType::from(base_value) + ValueType::from(delta));
            assert!(!(probability_outcome_two <= probability_outcome_one));
        }

        #[test]
        fn test_le_same(base_value: SmallValueType) {
            let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(base_value.into());
            let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(base_value.into());
            assert!(probability_outcome_two <= probability_outcome_one);
        }

        #[test]
        fn test_cmp_less_value(base_value: SmallValueType, delta: UnsignedSmallValueType) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(base_value.into());
            let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(ValueType::from(base_value) + ValueType::from(delta));
            assert_eq!(probability_outcome_one.cmp(&probability_outcome_two), Less);
            assert_eq!(probability_outcome_one.partial_cmp(&probability_outcome_two), Some(Less));
        }

        #[test]
        fn test_cmp_greater_value(base_value: SmallValueType, delta: UnsignedSmallValueType) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(ValueType::from(base_value) + ValueType::from(delta));
            let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(base_value.into());
            assert_eq!(probability_outcome_one.cmp(&probability_outcome_two), Greater);
            assert_eq!(probability_outcome_one.partial_cmp(&probability_outcome_two), Some(Greater));
        }

        #[test]
        fn test_cmp_equal_value(base_value: SmallValueType) {
            let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(base_value.into());
            let probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(base_value.into());
            assert_eq!(probability_outcome_one.cmp(&probability_outcome_two), Equal);
            assert_eq!(probability_outcome_one.partial_cmp(&probability_outcome_two), Some(Equal));
        }

        #[test]
        fn test_cmp_less_constraint(base_value: ConstraintIdType) {
            prop_assume!(base_value != ConstraintIdType::MAX);
            let probability_outcome_one = ProbabilityOutcome::new_with_constraints(
                321,
                vec![Constraint::new_empty_constraint(base_value)]
            );
            let probability_outcome_two = ProbabilityOutcome::new_with_constraints(
                321,
                vec![Constraint::new_empty_constraint(base_value + 1)]
            );
            assert_eq!(probability_outcome_one.cmp(&probability_outcome_two), Less);
            assert_eq!(probability_outcome_one.partial_cmp(&probability_outcome_two), Some(Less));
        }

        #[test]
        fn test_cmp_greater_constraint(base_value: ConstraintIdType) {
            prop_assume!(base_value != ConstraintIdType::MIN);
            let probability_outcome_one = ProbabilityOutcome::new_with_constraints(
                321,
                vec![Constraint::new_empty_constraint(base_value)]
            );
            let probability_outcome_two = ProbabilityOutcome::new_with_constraints(
                321,
                vec![Constraint::new_empty_constraint(base_value - 1)]
            );
            assert_eq!(probability_outcome_one.cmp(&probability_outcome_two), Greater);
            assert_eq!(probability_outcome_one.partial_cmp(&probability_outcome_two), Some(Greater));
        }

        #[test]
        fn test_cmp_equal_constraint(base_value: ConstraintIdType) {
            let probability_outcome_one = ProbabilityOutcome::new_with_constraints(
                321,
                vec![Constraint::new_empty_constraint(base_value)]
            );
            let probability_outcome_two = ProbabilityOutcome::new_with_constraints(
                321,
                vec![Constraint::new_empty_constraint(base_value)]
            );
            assert_eq!(probability_outcome_one.cmp(&probability_outcome_two), Equal);
            assert_eq!(probability_outcome_one.partial_cmp(&probability_outcome_two), Some(Equal));
        }

        #[test]
        fn test_cmp_less_value_before_constraint(base_value: SmallValueType, delta: UnsignedSmallValueType, constraint_value: ConstraintIdType) {
            prop_assume!(constraint_value != ConstraintIdType::MIN);
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome::new_with_constraints(
                base_value.into(),
                vec![Constraint::new_empty_constraint(constraint_value)]
            );
            let probability_outcome_two = ProbabilityOutcome::new_with_constraints(
                ValueType::from(base_value) + ValueType::from(delta),
                vec![Constraint::new_empty_constraint(constraint_value - 1)]
            );

            assert_eq!(probability_outcome_one.cmp(&probability_outcome_two), Less);
            assert_eq!(probability_outcome_one.partial_cmp(&probability_outcome_two), Some(Less));
        }

        #[test]
        fn test_cmp_greater_value_before_constraint(base_value: SmallValueType, delta: UnsignedSmallValueType, constraint_value: ConstraintIdType) {
            prop_assume!(constraint_value != ConstraintIdType::MAX);
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome::new_with_constraints(
                ValueType::from(base_value) + ValueType::from(delta),
                vec![Constraint::new_empty_constraint(constraint_value)]
            );
            let probability_outcome_two = ProbabilityOutcome::new_with_constraints(
                base_value.into(),
                vec![Constraint::new_empty_constraint(constraint_value + 1)]
            );

            assert_eq!(probability_outcome_one.cmp(&probability_outcome_two), Greater);
            assert_eq!(probability_outcome_one.partial_cmp(&probability_outcome_two), Some(Greater));
        }

        #[test]
        fn test_fmt(value: ValueType) {
            let probability_outcome = ProbabilityOutcome::new_with_empty_constraint_map(value);
            assert_eq!(format!("{probability_outcome:?}"), format!("ProbabilityOutcome {{ value: {}, constraint_map: ConstraintMap {{ map: {{}} }} }}", value));
        }
    }

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn test_clone() {
        let mut probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(10);
        let probability_outcome_two = probability_outcome_one.clone();

        assert_eq!(probability_outcome_one, probability_outcome_two);
        probability_outcome_one.value = 20;
        assert_ne!(probability_outcome_one, probability_outcome_two);
    }

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn test_clone_from() {
        let mut probability_outcome_two = ProbabilityOutcome::new_with_empty_constraint_map(2);
        let probability_outcome_one = ProbabilityOutcome::new_with_empty_constraint_map(10);
        probability_outcome_two.clone_from(&probability_outcome_one);
        assert_ne!(probability_outcome_two.value, 2);
    }
}
