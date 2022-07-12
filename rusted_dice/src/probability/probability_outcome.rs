use std::cmp::{Ord, Eq, PartialOrd};

#[derive(Copy, Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct ProbabilityOutcome {
    pub value: i32,
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_value_set(test_value: i32) {
            let probability_outcome = ProbabilityOutcome {value: test_value};
            assert!(probability_outcome.value == test_value);
        }

        #[test]
        fn test_eq_true(test_value: i32) {
            let probability_outcome_one = ProbabilityOutcome {value: test_value};
            let probability_outcome_two = ProbabilityOutcome {value: test_value};
            assert!(probability_outcome_one == probability_outcome_two);
        }

        #[test]
        #[allow(clippy::nonminimal_bool)]
        fn test_eq_false(test_value: i32, other_test_value: i32) {
            prop_assume!(test_value != other_test_value);
            let probability_outcome_one = ProbabilityOutcome {value: test_value};
            let probability_outcome_two = ProbabilityOutcome {value: other_test_value};
            assert!(!(probability_outcome_one == probability_outcome_two));
        }

        #[test]
        fn test_ne_true(test_value: i32, other_test_value: i32) {
            prop_assume!(test_value != other_test_value);
            let probability_outcome_one = ProbabilityOutcome {value: test_value};
            let probability_outcome_two = ProbabilityOutcome {value: other_test_value};
            assert!(probability_outcome_one != probability_outcome_two);
        }
                
        #[test]
        #[allow(clippy::nonminimal_bool)]
        fn test_ne_false(test_value: i32) {
            let probability_outcome_one = ProbabilityOutcome {value: test_value};
            let probability_outcome_two = ProbabilityOutcome {value: test_value};
            assert!(!(probability_outcome_one != probability_outcome_two));
        }

        #[test]
        fn test_gt_true(base_value: i16, delta: u16) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome {value: base_value.into()};
            let probability_outcome_two = ProbabilityOutcome {value: i32::from(base_value) + i32::from(delta)};
            assert!(probability_outcome_two > probability_outcome_one);
        }
        
        #[test]
        #[allow(clippy::nonminimal_bool)]
        fn test_gt_false(base_value: i16, delta: u16) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome {value: base_value.into()};
            let probability_outcome_two = ProbabilityOutcome {value: i32::from(base_value) + i32::from(delta)};
            assert!(!(probability_outcome_one > probability_outcome_two));
        }
        
        #[test]
        #[allow(clippy::nonminimal_bool)]
        fn test_gt_same(base_value: i16) {
            let probability_outcome_one = ProbabilityOutcome {value: base_value.into()};
            let probability_outcome_two = ProbabilityOutcome {value: base_value.into()};
            assert!(!(probability_outcome_two > probability_outcome_one));
        }
        
        #[test]
        fn test_lt_true(base_value: i16, delta: u16) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome {value: base_value.into()};
            let probability_outcome_two = ProbabilityOutcome {value: i32::from(base_value) + i32::from(delta)};
            assert!(probability_outcome_one < probability_outcome_two);
        }
        
        #[test]
        #[allow(clippy::nonminimal_bool)]
        fn test_lt_false(base_value: i16, delta: u16) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome {value: base_value.into()};
            let probability_outcome_two = ProbabilityOutcome {value: i32::from(base_value) + i32::from(delta)};
            assert!(!(probability_outcome_two < probability_outcome_one));
        }
        
        #[test]
        #[allow(clippy::nonminimal_bool)]
        fn test_lt_same(base_value: i16) {
            let probability_outcome_one = ProbabilityOutcome {value: base_value.into()};
            let probability_outcome_two = ProbabilityOutcome {value: base_value.into()};
            assert!(!(probability_outcome_two < probability_outcome_one));
        }
        
        #[test]
        fn test_ge_true(base_value: i16, delta: u16) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome {value: base_value.into()};
            let probability_outcome_two = ProbabilityOutcome {value: i32::from(base_value) + i32::from(delta)};
            assert!(probability_outcome_two >= probability_outcome_one);
        }
        
        #[test]
        #[allow(clippy::nonminimal_bool)]
        fn test_ge_false(base_value: i16, delta: u16) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome {value: base_value.into()};
            let probability_outcome_two = ProbabilityOutcome {value: i32::from(base_value) + i32::from(delta)};
            assert!(!(probability_outcome_one >= probability_outcome_two));
        }
        
        #[test]
        fn test_ge_same(base_value: i16) {
            let probability_outcome_one = ProbabilityOutcome {value: base_value.into()};
            let probability_outcome_two = ProbabilityOutcome {value: base_value.into()};
            assert!(probability_outcome_two >= probability_outcome_one);
        }

        #[test]
        fn test_le_true(base_value: i16, delta: u16) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome {value: base_value.into()};
            let probability_outcome_two = ProbabilityOutcome {value: i32::from(base_value) + i32::from(delta)};
            assert!(probability_outcome_one <= probability_outcome_two);
        }
        
        #[test]
        #[allow(clippy::nonminimal_bool)]
        fn test_le_false(base_value: i16, delta: u16) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome {value: base_value.into()};
            let probability_outcome_two = ProbabilityOutcome {value: i32::from(base_value) + i32::from(delta)};
            assert!(!(probability_outcome_two <= probability_outcome_one));
        }
        
        #[test]
        fn test_le_same(base_value: i16) {
            let probability_outcome_one = ProbabilityOutcome {value: base_value.into()};
            let probability_outcome_two = ProbabilityOutcome {value: base_value.into()};
            assert!(probability_outcome_two <= probability_outcome_one);
        }
    
        #[test]
        fn test_cmp_less(base_value: i16, delta: u16) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome {value: base_value.into()};
            let probability_outcome_two = ProbabilityOutcome {value: i32::from(base_value) + i32::from(delta)};
            let result = probability_outcome_one.cmp(&probability_outcome_two);
            assert_eq!(result, Ordering::Less);
        }
            
        #[test]
        fn test_cmp_greater(base_value: i16, delta: u16) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome {value: i32::from(base_value) + i32::from(delta)};
            let probability_outcome_two = ProbabilityOutcome {value: base_value.into()};
            let result = probability_outcome_one.cmp(&probability_outcome_two);
            assert_eq!(result, Ordering::Greater);
        }
            
        #[test]
        fn test_cmp_equal(base_value: i16) {
            let probability_outcome_one = ProbabilityOutcome {value: base_value.into()};
            let probability_outcome_two = ProbabilityOutcome {value: base_value.into()};
            let result = probability_outcome_one.cmp(&probability_outcome_two);
            assert_eq!(result, Ordering::Equal);
        }
                
        #[test]
        fn test_fmt(value: i32) {
            let probability_outcome = ProbabilityOutcome {value};
            assert_eq!(format!("{probability_outcome:?}"), format!("ProbabilityOutcome {{ value: {} }}", value));
        }
    
    
    }

    #[test]
    fn test_copy() {
        let mut probability_outcome_one = ProbabilityOutcome {value: 10};
        let probability_outcome_two = probability_outcome_one;
        probability_outcome_one.value = 20;
        assert_ne!(probability_outcome_one, probability_outcome_two);
    }

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn test_clone() {
        let mut probability_outcome_one = ProbabilityOutcome {value: 10};
        let probability_outcome_two = probability_outcome_one.clone();
        probability_outcome_one.value = 20;
        assert_ne!(probability_outcome_one, probability_outcome_two);
    }
    
    #[test]
    #[allow(clippy::clone_on_copy)]
    fn test_clone_from() {
        let mut probability_outcome_two = ProbabilityOutcome {value: 2};
        let probability_outcome_one = ProbabilityOutcome {value: 10};
        probability_outcome_two.clone_from(&probability_outcome_one);
        assert_ne!(probability_outcome_two.value, 2);
    }
}
