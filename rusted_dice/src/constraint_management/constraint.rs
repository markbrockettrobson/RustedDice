use std::collections::HashSet;

type I32Set = HashSet<i32>;

#[allow(dead_code)]
#[derive(Debug)]
struct Constraint {
    pub id: u16,
    pub valid_values: I32Set,
}

trait IsPossable {
    fn is_possable(&self) -> bool;
}

impl IsPossable for Constraint {
    fn is_possable(&self) -> bool {
        !self.valid_values.is_empty()
    }
}

trait Combine {
    fn combine(&self, other: Self) -> Self;
}

impl Combine for Constraint {
    fn combine(&self, other: Self) -> Self {
        if self.id != other.id {
            panic!("Can not combine Constraints with different ids.");
        }
        Constraint {id: self.id, valid_values: self.valid_values.union(&other.valid_values).copied().collect()}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn is_possable_true() {
        let test_valid_values: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
        let constraint = Constraint {id: 0, valid_values: test_valid_values };

        assert!(constraint.is_possable());
    }
    
    #[test]
    fn is_possable_false() {
        let test_valid_values: HashSet<i32> = vec![].into_iter().collect();
        let constraint = Constraint {id: 0, valid_values: test_valid_values };

        assert!(!constraint.is_possable());
    }

    #[test]
    #[should_panic(expected = "Can not combine Constraints with different ids.")]
    fn panic_on_different_id_combine() {
        let test_valid_value_one: HashSet<i32> = vec![].into_iter().collect();
        let test_valid_value_two: HashSet<i32> = vec![].into_iter().collect();
        let constraint_one = Constraint {id: 0, valid_values: test_valid_value_one };
        let constraint_two = Constraint {id: 1, valid_values: test_valid_value_two };
        constraint_one.combine(constraint_two);
    }

    #[test]
    fn combine() {
        let test_valid_value_one: HashSet<i32> = vec![1, 3, 5].into_iter().collect();
        let test_valid_value_two: HashSet<i32> = vec![2, 4, 6].into_iter().collect();
        let expected_value: HashSet<i32> = vec![1, 2, 3, 4, 5, 6].into_iter().collect();
        let constraint_one = Constraint {id: 1234, valid_values: test_valid_value_one };
        let constraint_two = Constraint {id: 1234, valid_values: test_valid_value_two };
        
        let constraint_three = constraint_one.combine(constraint_two);
        
        assert_eq!(constraint_three.valid_values.difference(&expected_value).count(), 0);
        assert_eq!(constraint_three.id, 1234);
    }

    proptest! {                        
        #[test]
        fn test_fmt(test_id: u16, test_value: i32) {
            let test_valid_values: HashSet<i32> = vec![test_value].into_iter().collect();
            let constraint = Constraint {id: test_id, valid_values: test_valid_values };
            assert_eq!(format!("{constraint:?}"), format!("Constraint {{ id: {}, valid_values: {{{}}} }}", test_id, test_value));
        }
    }
}