use std::collections::HashSet;
use super::constraint::Constraint;

#[allow(dead_code)]
pub(crate) struct ConstraintFactory;

#[allow(dead_code)]
impl ConstraintFactory {
    fn new_empty_constraint(id: u16) -> Constraint {
        let valid_values: HashSet<i32> = vec![].into_iter().collect();
        Constraint {id, valid_values}
    }

    fn new_single_valid_value_constraint(id: u16, value: i32) -> Constraint {
        let valid_values: HashSet<i32> = vec![value].into_iter().collect();
        Constraint {id, valid_values}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_empty_constraint() {
        let test_valid_values: HashSet<i32> = vec![].into_iter().collect();
        let constraint = Constraint {id: 12345, valid_values: test_valid_values };

        assert_eq!(constraint, ConstraintFactory::new_empty_constraint(12345));
    }  

    #[test]
    fn test_new_single_valid_value_constraint() {
        let test_valid_values: HashSet<i32> = vec![654321].into_iter().collect();
        let constraint = Constraint {id: 12345, valid_values: test_valid_values };

        assert_eq!(constraint, ConstraintFactory::new_single_valid_value_constraint(12345, 654321));
    }
}
