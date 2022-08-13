use std::cmp::{Ord, Eq, PartialOrd};
use std::ops::{Add, Sub, Mul, Div, BitOr, BitXor, BitAnd, Not, Neg, Rem};

use super::super::constraint_management::constraint_map::ConstraintMap;

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub(crate) struct ProbabilityOutcome {
    pub value: i32,
    pub constraint_map: ConstraintMap,
}

type BinaryOperation = fn(i32, i32) -> i32;

trait Combine {
    fn combine(&self, other: Self, binary_operation: BinaryOperation) -> Self;
    fn combinei32(&self, other: i32, binary_operation: BinaryOperation) -> Self;
    fn i32combine(&self, other: i32, binary_operation: BinaryOperation) -> Self;
}

impl Combine for ProbabilityOutcome {
    fn combine(&self, other: Self, binary_operation: BinaryOperation) -> Self {
        ProbabilityOutcome {value: binary_operation(self.value, other.value), constraint_map: self.constraint_map.clone() + other.constraint_map}
    }
    fn combinei32(&self, other: i32, binary_operation: BinaryOperation) -> Self {
        ProbabilityOutcome {value: binary_operation(self.value, other), constraint_map: self.constraint_map.clone()}
    }
    fn i32combine(&self, other: i32, binary_operation: BinaryOperation) -> Self {
        ProbabilityOutcome {value: binary_operation(other, self.value), constraint_map: self.constraint_map.clone()}
    }
}

impl Add for ProbabilityOutcome {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        fn _add(lhs: i32, rhs: i32) -> i32{
            lhs + rhs
        }
        self.combine(other, _add)
    }
}

impl Add<i32> for ProbabilityOutcome {
    type Output = Self;

    fn add(self, other: i32) -> Self {
        fn _add(lhs: i32, rhs: i32) -> i32{
            lhs + rhs
        }
        self.combinei32(other, _add)
    }
}

impl Add<ProbabilityOutcome> for i32 {
    type Output = ProbabilityOutcome;

    fn add(self, other: ProbabilityOutcome) -> ProbabilityOutcome {
        fn _add(lhs: i32, rhs: i32) -> i32{
            lhs + rhs
        }
        other.i32combine(self, _add)
    }
}

impl Sub for ProbabilityOutcome {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        fn _sub(lhs: i32, rhs: i32) -> i32{
            lhs - rhs
        }
        self.combine(other, _sub)
    }
}

impl Sub<i32> for ProbabilityOutcome {
    type Output = Self;

    fn sub(self, other: i32) -> Self {
        fn _sub(lhs: i32, rhs: i32) -> i32{
            lhs - rhs
        }
        self.combinei32(other, _sub)
    }
}

impl Sub<ProbabilityOutcome> for i32 {
    type Output = ProbabilityOutcome;

    fn sub(self, other: ProbabilityOutcome) -> ProbabilityOutcome {
        fn _sub(lhs: i32, rhs: i32) -> i32{
            lhs - rhs
        }
        other.i32combine(self, _sub)
    }
}

impl Mul for ProbabilityOutcome {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        fn _mul(lhs: i32, rhs: i32) -> i32{
            lhs * rhs
        }
        self.combine(other, _mul)
    }
}

impl Mul<i32> for ProbabilityOutcome {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        fn _mul(lhs: i32, rhs: i32) -> i32{
            lhs * rhs
        }
        self.combinei32(other, _mul)
    }
}

impl Mul<ProbabilityOutcome> for i32 {
    type Output = ProbabilityOutcome;

    fn mul(self, other: ProbabilityOutcome) -> ProbabilityOutcome {
        fn _mul(lhs: i32, rhs: i32) -> i32{
            lhs * rhs
        }
        other.i32combine(self, _mul)
    }
}

impl Div for ProbabilityOutcome {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        fn _div(lhs: i32, rhs: i32) -> i32{
            lhs / rhs
        }
        self.combine(other, _div)
    }
}

impl Div<i32> for ProbabilityOutcome {
    type Output = Self;

    fn div(self, other: i32) -> Self {
        fn _div(lhs: i32, rhs: i32) -> i32{
            lhs / rhs
        }
        self.combinei32(other, _div)
    }
}

impl Div<ProbabilityOutcome> for i32 {
    type Output = ProbabilityOutcome;

    fn div(self, other: ProbabilityOutcome) -> ProbabilityOutcome {
        fn _div(lhs: i32, rhs: i32) -> i32{
            lhs / rhs
        }
        other.i32combine(self, _div)
    }
}

impl Rem for ProbabilityOutcome {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        fn _rem(lhs: i32, rhs: i32) -> i32{
            lhs % rhs
        }
        self.combine(other, _rem)
    }
}

impl Rem<i32> for ProbabilityOutcome {
    type Output = Self;

    fn rem(self, other: i32) -> Self {
        fn _rem(lhs: i32, rhs: i32) -> i32{
            lhs % rhs
        }
        self.combinei32(other, _rem)
    }
}

impl Rem<ProbabilityOutcome> for i32 {
    type Output = ProbabilityOutcome;

    fn rem(self, other: ProbabilityOutcome) -> ProbabilityOutcome {
        fn _rem(lhs: i32, rhs: i32) -> i32{
            lhs % rhs
        }
        other.i32combine(self, _rem)
    }
}

impl BitOr for ProbabilityOutcome {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        fn _bitor(lhs: i32, rhs: i32) -> i32{
            lhs | rhs
        }
        self.combine(other, _bitor)
    }
}

impl BitOr<i32> for ProbabilityOutcome {
    type Output = Self;

    fn bitor(self, other: i32) -> Self {
        fn _bitor(lhs: i32, rhs: i32) -> i32{
            lhs | rhs
        }
        self.combinei32(other, _bitor)
    }
}

impl BitOr<ProbabilityOutcome> for i32 {
    type Output = ProbabilityOutcome;

    fn bitor(self, other: ProbabilityOutcome) -> ProbabilityOutcome {
        fn _bitor(lhs: i32, rhs: i32) -> i32{
            lhs | rhs
        }
        other.i32combine(self, _bitor)
    }
}

impl BitXor for ProbabilityOutcome {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        fn _bitxor(lhs: i32, rhs: i32) -> i32{
            lhs ^ rhs
        }
        self.combine(other, _bitxor)
    }
}

impl BitXor<i32> for ProbabilityOutcome {
    type Output = Self;

    fn bitxor(self, other: i32) -> Self {
        fn _bitxor(lhs: i32, rhs: i32) -> i32{
            lhs ^ rhs
        }
        self.combinei32(other, _bitxor)
    }
}

impl BitXor<ProbabilityOutcome> for i32 {
    type Output = ProbabilityOutcome;

    fn bitxor(self, other: ProbabilityOutcome) -> ProbabilityOutcome {
        fn _bitxor(lhs: i32, rhs: i32) -> i32{
            lhs ^ rhs
        }
        other.i32combine(self, _bitxor)
    }
}

impl BitAnd for ProbabilityOutcome {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        fn _bitand(lhs: i32, rhs: i32) -> i32{
            lhs & rhs
        }
        self.combine(other, _bitand)
    }
}

impl BitAnd<i32> for ProbabilityOutcome {
    type Output = Self;

    fn bitand(self, other: i32) -> Self {
        fn _bitand(lhs: i32, rhs: i32) -> i32{
            lhs & rhs
        }
        self.combinei32(other, _bitand)
    }
}

impl BitAnd<ProbabilityOutcome> for i32 {
    type Output = ProbabilityOutcome;

    fn bitand(self, other: ProbabilityOutcome) -> ProbabilityOutcome {
        fn _bitand(lhs: i32, rhs: i32) -> i32{
            lhs & rhs
        }
        other.i32combine(self, _bitand)
    }
}

impl Not for ProbabilityOutcome {
    type Output = Self;

    fn not(self) -> Self {
        Self { value: !self.value, constraint_map: self.constraint_map }
    }
}

impl Neg for ProbabilityOutcome {
    type Output = Self;

    fn neg(self) -> Self {
        Self { value: -self.value, constraint_map: self.constraint_map }
    }
}

#[cfg(test)]
mod tests {
    use std::{cmp::Ordering, collections::HashMap};
    
    use crate::constraint_management::constraint::Constraint;

    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_combine_constraint_map() {
        let constraint1_12 = Constraint {id: 1, valid_values: vec![1, 2].into_iter().collect()};
        let constraint1_23 = Constraint {id: 1, valid_values: vec![2, 3].into_iter().collect()};
        let mut map_one: HashMap<u16, Constraint> = HashMap::new();
        map_one.insert(1, constraint1_12);
        let mut map_two: HashMap<u16, Constraint> = HashMap::new();
        map_two.insert(1, constraint1_23);
        let test_constraint_map_one = ConstraintMap {map: map_one};
        let test_constraint_map_two = ConstraintMap {map: map_two};

        let probability_outcome_one = ProbabilityOutcome {value: 123, constraint_map: test_constraint_map_one};
        let probability_outcome_two = ProbabilityOutcome {value: 123, constraint_map: test_constraint_map_two};


        let combined_probability_outcome = probability_outcome_one.combine(probability_outcome_two, |lhs, rhs| {
            lhs + rhs
        });

        let combined_constraint_map = combined_probability_outcome.constraint_map;

        assert_eq!(combined_constraint_map.map.len(), 1);
    }

    #[test]
    fn test_combine_constrainti32_map() {
        let constraint1_12 = Constraint {id: 1, valid_values: vec![1, 2].into_iter().collect()};
        let mut map: HashMap<u16, Constraint> = HashMap::new();
        map.insert(1, constraint1_12);

        let probability_outcome = ProbabilityOutcome {value: 123, constraint_map: ConstraintMap { map: map.clone() }};

        let combined_probability_outcome = probability_outcome.combinei32(1, |lhs, rhs| {
            lhs + rhs
        });

        let combined_constraint_map = combined_probability_outcome.constraint_map;

        assert_eq!(combined_constraint_map.map, map);
    }

    #[test]
    fn test_combine_i32constraint_map() {
        let constraint1_12 = Constraint {id: 1, valid_values: vec![1, 2].into_iter().collect()};
        let mut map: HashMap<u16, Constraint> = HashMap::new();
        map.insert(1, constraint1_12);

        let probability_outcome = ProbabilityOutcome {value: 123, constraint_map: ConstraintMap { map: map.clone() }};

        let combined_probability_outcome = probability_outcome.i32combine(1, |lhs, rhs| {
            lhs + rhs
        });

        let combined_constraint_map = combined_probability_outcome.constraint_map;

        assert_eq!(combined_constraint_map.map, map);
    }

    proptest! {
        #[test]
        fn test_value_set(test_value: i32) {
            let probability_outcome = ProbabilityOutcome {value: test_value,  constraint_map: ConstraintMap {map: HashMap::new()}};
            assert!(probability_outcome.value == test_value);
        }

        #[test]
        fn test_constraint_map_set(test_value: u16) {
            let constraint1_123 = Constraint {id: test_value, valid_values: vec![1, 2, 3].into_iter().collect()};
            let mut map: HashMap<u16, Constraint> = HashMap::new();
            map.insert(test_value, constraint1_123);
            let test_constraint_map = ConstraintMap {map};

            let probability_outcome = ProbabilityOutcome {value: 123, constraint_map: test_constraint_map};

            assert!(probability_outcome.constraint_map.map.contains_key(&test_value));
            assert!(probability_outcome.constraint_map.map.get(&test_value).unwrap().valid_values.contains(&1));
            assert!(probability_outcome.constraint_map.map.get(&test_value).unwrap().valid_values.contains(&2));
            assert!(probability_outcome.constraint_map.map.get(&test_value).unwrap().valid_values.contains(&3));
        }

        #[test]
        fn test_eq_true(test_value: u16) {
            let constraint1_123 = Constraint {id: test_value, valid_values: vec![1, 2, 3].into_iter().collect()};
            let mut map: HashMap<u16, Constraint> = HashMap::new();
            map.insert(test_value, constraint1_123);
            let test_constraint_map = ConstraintMap {map};
            
            let probability_outcome_one = ProbabilityOutcome {value: 123,  constraint_map: test_constraint_map.clone()};
            let probability_outcome_two = ProbabilityOutcome {value: 123,  constraint_map: test_constraint_map};
            assert!(probability_outcome_one == probability_outcome_two);
        }

        #[test]
        #[allow(clippy::nonminimal_bool)]
        fn test_eq_false(test_value: i32, other_test_value: i32) {
            prop_assume!(test_value != other_test_value);
            let probability_outcome_one = ProbabilityOutcome {value: test_value,  constraint_map: ConstraintMap {map: HashMap::new()}};
            let probability_outcome_two = ProbabilityOutcome {value: other_test_value,  constraint_map: ConstraintMap {map: HashMap::new()}};
            assert!(!(probability_outcome_one == probability_outcome_two));
        }

        #[test]
        fn test_ne_true(test_value: i32, other_test_value: i32) {
            prop_assume!(test_value != other_test_value);
            let probability_outcome_one = ProbabilityOutcome {value: test_value,  constraint_map: ConstraintMap {map: HashMap::new()}};
            let probability_outcome_two = ProbabilityOutcome {value: other_test_value,  constraint_map: ConstraintMap {map: HashMap::new()}};
            assert!(probability_outcome_one != probability_outcome_two);
        }
                
        #[test]
        #[allow(clippy::nonminimal_bool)]
        fn test_ne_false(test_value: i32) {
            let probability_outcome_one = ProbabilityOutcome {value: test_value,  constraint_map: ConstraintMap {map: HashMap::new()}};
            let probability_outcome_two = ProbabilityOutcome {value: test_value,  constraint_map: ConstraintMap {map: HashMap::new()}};
            assert!(!(probability_outcome_one != probability_outcome_two));
        }

        #[test]
        fn test_gt_true(base_value: i16, delta: u16) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome {value: base_value.into(),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let probability_outcome_two = ProbabilityOutcome {value: i32::from(base_value) + i32::from(delta),  constraint_map: ConstraintMap {map: HashMap::new()}};
            assert!(probability_outcome_two > probability_outcome_one);
        }
        
        #[test]
        #[allow(clippy::nonminimal_bool)]
        fn test_gt_false(base_value: i16, delta: u16) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome {value: base_value.into(),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let probability_outcome_two = ProbabilityOutcome {value: i32::from(base_value) + i32::from(delta),  constraint_map: ConstraintMap {map: HashMap::new()}};
            assert!(!(probability_outcome_one > probability_outcome_two));
        }
        
        #[test]
        #[allow(clippy::nonminimal_bool)]
        fn test_gt_same(base_value: i16) {
            let probability_outcome_one = ProbabilityOutcome {value: base_value.into(),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let probability_outcome_two = ProbabilityOutcome {value: base_value.into(),  constraint_map: ConstraintMap {map: HashMap::new()}};
            assert!(!(probability_outcome_two > probability_outcome_one));
        }
        
        #[test]
        fn test_lt_true(base_value: i16, delta: u16) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome {value: base_value.into(),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let probability_outcome_two = ProbabilityOutcome {value: i32::from(base_value) + i32::from(delta),  constraint_map: ConstraintMap {map: HashMap::new()}};
            assert!(probability_outcome_one < probability_outcome_two);
        }
        
        #[test]
        #[allow(clippy::nonminimal_bool)]
        fn test_lt_false(base_value: i16, delta: u16) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome {value: base_value.into(),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let probability_outcome_two = ProbabilityOutcome {value: i32::from(base_value) + i32::from(delta),  constraint_map: ConstraintMap {map: HashMap::new()}};
            assert!(!(probability_outcome_two < probability_outcome_one));
        }
        
        #[test]
        #[allow(clippy::nonminimal_bool)]
        fn test_lt_same(base_value: i16) {
            let probability_outcome_one = ProbabilityOutcome {value: base_value.into(),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let probability_outcome_two = ProbabilityOutcome {value: base_value.into(),  constraint_map: ConstraintMap {map: HashMap::new()}};
            assert!(!(probability_outcome_two < probability_outcome_one));
        }
        
        #[test]
        fn test_ge_true(base_value: i16, delta: u16) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome {value: base_value.into(),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let probability_outcome_two = ProbabilityOutcome {value: i32::from(base_value) + i32::from(delta),  constraint_map: ConstraintMap {map: HashMap::new()}};
            assert!(probability_outcome_two >= probability_outcome_one);
        }
        
        #[test]
        #[allow(clippy::nonminimal_bool)]
        fn test_ge_false(base_value: i16, delta: u16) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome {value: base_value.into(),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let probability_outcome_two = ProbabilityOutcome {value: i32::from(base_value) + i32::from(delta),  constraint_map: ConstraintMap {map: HashMap::new()}};
            assert!(!(probability_outcome_one >= probability_outcome_two));
        }
        
        #[test]
        fn test_ge_same(base_value: i16) {
            let probability_outcome_one = ProbabilityOutcome {value: base_value.into(),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let probability_outcome_two = ProbabilityOutcome {value: base_value.into(),  constraint_map: ConstraintMap {map: HashMap::new()}};
            assert!(probability_outcome_two >= probability_outcome_one);
        }

        #[test]
        fn test_le_true(base_value: i16, delta: u16) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome {value: base_value.into(),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let probability_outcome_two = ProbabilityOutcome {value: i32::from(base_value) + i32::from(delta),  constraint_map: ConstraintMap {map: HashMap::new()}};
            assert!(probability_outcome_one <= probability_outcome_two);
        }
        
        #[test]
        #[allow(clippy::nonminimal_bool)]
        fn test_le_false(base_value: i16, delta: u16) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome {value: base_value.into(),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let probability_outcome_two = ProbabilityOutcome {value: i32::from(base_value) + i32::from(delta),  constraint_map: ConstraintMap {map: HashMap::new()}};
            assert!(!(probability_outcome_two <= probability_outcome_one));
        }
        
        #[test]
        fn test_le_same(base_value: i16) {
            let probability_outcome_one = ProbabilityOutcome {value: base_value.into(),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let probability_outcome_two = ProbabilityOutcome {value: base_value.into(),  constraint_map: ConstraintMap {map: HashMap::new()}};
            assert!(probability_outcome_two <= probability_outcome_one);
        }
    
        #[test]
        fn test_cmp_less(base_value: i16, delta: u16) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome {value: base_value.into(),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let probability_outcome_two = ProbabilityOutcome {value: i32::from(base_value) + i32::from(delta),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = probability_outcome_one.cmp(&probability_outcome_two);
            assert_eq!(result, Ordering::Less);
        }
            
        #[test]
        fn test_cmp_greater(base_value: i16, delta: u16) {
            prop_assume!(delta != 0);
            let probability_outcome_one = ProbabilityOutcome {value: i32::from(base_value) + i32::from(delta),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let probability_outcome_two = ProbabilityOutcome {value: base_value.into(),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = probability_outcome_one.cmp(&probability_outcome_two);
            assert_eq!(result, Ordering::Greater);
        }
            
        #[test]
        fn test_cmp_equal(base_value: i16) {
            let probability_outcome_one = ProbabilityOutcome {value: base_value.into(),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let probability_outcome_two = ProbabilityOutcome {value: base_value.into(),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = probability_outcome_one.cmp(&probability_outcome_two);
            assert_eq!(result, Ordering::Equal);
        }
                
        #[test]
        fn test_fmt(value: i32) {
            let constraint_map = ConstraintMap {map: HashMap::new()};
            let probability_outcome = ProbabilityOutcome {value,  constraint_map: constraint_map.clone()};
            assert_eq!(format!("{probability_outcome:?}"), format!("ProbabilityOutcome {{ value: {}, constraint_map: {:?} }}", value, constraint_map));
        }

        #[test]
        fn test_add(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) + i32::from(value_two);
            let probability_outcome_one = ProbabilityOutcome {value: i32::from(value_one),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let probability_outcome_two = ProbabilityOutcome {value: i32::from(value_two),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = probability_outcome_one + probability_outcome_two;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_add_i32(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) + i32::from(value_two);
            let probability_outcome = ProbabilityOutcome {value: i32::from(value_one),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = probability_outcome + i32::from(value_two);
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_i32_add(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) + i32::from(value_two);
            let probability_outcome = ProbabilityOutcome {value: i32::from(value_one),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = i32::from(value_two) + probability_outcome ;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_sub(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) - i32::from(value_two);
            let probability_outcome_one = ProbabilityOutcome {value: i32::from(value_one),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let probability_outcome_two = ProbabilityOutcome {value: i32::from(value_two),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = probability_outcome_one - probability_outcome_two;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_sub_i32(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) - i32::from(value_two);
            let probability_outcome = ProbabilityOutcome {value: i32::from(value_one),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = probability_outcome - i32::from(value_two);
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_i32_sub(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) - i32::from(value_two);
            let probability_outcome = ProbabilityOutcome {value: i32::from(value_two),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = i32::from(value_one) - probability_outcome;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_mul(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) * i32::from(value_two);
            let probability_outcome_one = ProbabilityOutcome {value: i32::from(value_one),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let probability_outcome_two = ProbabilityOutcome {value: i32::from(value_two),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = probability_outcome_one * probability_outcome_two;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_mul_i32(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) * i32::from(value_two);
            let probability_outcome = ProbabilityOutcome {value: i32::from(value_one),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = probability_outcome * i32::from(value_two);
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_i32_mul(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) * i32::from(value_two);
            let probability_outcome = ProbabilityOutcome {value: i32::from(value_two),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = i32::from(value_one) * probability_outcome;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_div(value_one: i32, value_two: i16) {
            prop_assume!(value_two != 0);
            let expected_value = value_one / i32::from(value_two);
            let probability_outcome_one = ProbabilityOutcome {value: value_one,  constraint_map: ConstraintMap {map: HashMap::new()}};
            let probability_outcome_two = ProbabilityOutcome {value: i32::from(value_two),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = probability_outcome_one / probability_outcome_two;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_div_i32(value_one: i16, value_two: i16) {
            prop_assume!(value_two != 0);
            let expected_value = i32::from(value_one) / i32::from(value_two);
            let probability_outcome = ProbabilityOutcome {value: i32::from(value_one),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = probability_outcome / i32::from(value_two);
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_i32_div(value_one: i16, value_two: i16) {
            prop_assume!(value_two != 0);
            let expected_value = i32::from(value_one) / i32::from(value_two);
            let probability_outcome = ProbabilityOutcome {value: i32::from(value_two),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = i32::from(value_one) / probability_outcome;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_rem(value_one: i32, value_two: i16) {
            prop_assume!(value_two != 0);
            let expected_value = value_one % i32::from(value_two);
            let probability_outcome_one = ProbabilityOutcome {value: value_one,  constraint_map: ConstraintMap {map: HashMap::new()}};
            let probability_outcome_two = ProbabilityOutcome {value: i32::from(value_two),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = probability_outcome_one % probability_outcome_two;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_rem_i32(value_one: i16, value_two: i16) {
            prop_assume!(value_two != 0);
            let expected_value = i32::from(value_one) % i32::from(value_two);
            let probability_outcome = ProbabilityOutcome {value: i32::from(value_one),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = probability_outcome % i32::from(value_two);
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_i32_rem(value_one: i16, value_two: i16) {
            prop_assume!(value_two != 0);
            let expected_value = i32::from(value_one) % i32::from(value_two);
            let probability_outcome = ProbabilityOutcome {value: i32::from(value_two),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = i32::from(value_one) % probability_outcome;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_bitor(value_one: i32, value_two: i32) {
            let expected_value = value_one | value_two;
            let probability_outcome_one = ProbabilityOutcome {value: value_one,  constraint_map: ConstraintMap {map: HashMap::new()}};
            let probability_outcome_two = ProbabilityOutcome {value: value_two,  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = probability_outcome_one | probability_outcome_two;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_bitor_i32(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) | i32::from(value_two);
            let probability_outcome = ProbabilityOutcome {value: i32::from(value_one),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = probability_outcome | i32::from(value_two);
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_i32_bitor(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) | i32::from(value_two);
            let probability_outcome = ProbabilityOutcome {value: i32::from(value_two),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = i32::from(value_one) | probability_outcome;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_bitxor(value_one: i32, value_two: i32) {
            let expected_value = value_one ^ value_two;
            let probability_outcome_one = ProbabilityOutcome {value: value_one,  constraint_map: ConstraintMap {map: HashMap::new()}};
            let probability_outcome_two = ProbabilityOutcome {value: value_two,  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = probability_outcome_one ^ probability_outcome_two;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_bitxor_i32(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) ^ i32::from(value_two);
            let probability_outcome = ProbabilityOutcome {value: i32::from(value_one),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = probability_outcome ^ i32::from(value_two);
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_i32_bitxor(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) ^ i32::from(value_two);
            let probability_outcome = ProbabilityOutcome {value: i32::from(value_two),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = i32::from(value_one) ^ probability_outcome;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_bitand(value_one: i32, value_two: i32) {
            let expected_value = value_one & value_two;
            let probability_outcome_one = ProbabilityOutcome {value: value_one,  constraint_map: ConstraintMap {map: HashMap::new()}};
            let probability_outcome_two = ProbabilityOutcome {value: value_two,  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = probability_outcome_one & probability_outcome_two;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_bitand_i32(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) & i32::from(value_two);
            let probability_outcome = ProbabilityOutcome {value: i32::from(value_one),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = probability_outcome & i32::from(value_two);
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_i32_bitand(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) & i32::from(value_two);
            let probability_outcome = ProbabilityOutcome {value: i32::from(value_two),  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = i32::from(value_one) & probability_outcome;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_not(value_one: i32) {
            let expected_value = !value_one;
            let probability_outcome = ProbabilityOutcome {value: value_one,  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = !probability_outcome;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_neg(value_one: i32) {
            let expected_value = -value_one;
            let probability_outcome = ProbabilityOutcome {value: value_one,  constraint_map: ConstraintMap {map: HashMap::new()}};
            let result = -probability_outcome;
            assert_eq!(result.value, expected_value);
        }
    }

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn test_clone() {
        let mut probability_outcome_one = ProbabilityOutcome {value: 10,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let probability_outcome_two = probability_outcome_one.clone();
        
        assert_eq!(probability_outcome_one, probability_outcome_two);
        probability_outcome_one.value = 20;
        assert_ne!(probability_outcome_one, probability_outcome_two);
    }
    
    #[test]
    #[allow(clippy::clone_on_copy)]
    fn test_clone_from() {
        let mut probability_outcome_two = ProbabilityOutcome {value: 2,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let probability_outcome_one = ProbabilityOutcome {value: 10,  constraint_map: ConstraintMap {map: HashMap::new()}};
        probability_outcome_two.clone_from(&probability_outcome_one);
        assert_ne!(probability_outcome_two.value, 2);
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_add_overflow() {
        let probability_outcome_one = ProbabilityOutcome {value: i32::MAX,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let probability_outcome_two = ProbabilityOutcome {value: 1,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let _ = probability_outcome_one + probability_outcome_two;
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_add_underflow() {
        let probability_outcome_one = ProbabilityOutcome {value: i32::MIN,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let probability_outcome_two = ProbabilityOutcome {value: -1,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let _ = probability_outcome_one + probability_outcome_two;
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_add_i32_overflow() {
        let probability_outcome = ProbabilityOutcome {value: i32::MAX,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let _ = probability_outcome + 1;
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_add_i32_underflow() {
        let probability_outcome = ProbabilityOutcome {value: i32::MIN,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let _ = probability_outcome + -1;
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_i32_add_overflow() {
        let probability_outcome = ProbabilityOutcome {value: i32::MAX,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let _ = 1 + probability_outcome;
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_i32_add_underflow() {
        let probability_outcome = ProbabilityOutcome {value: i32::MIN,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let _ = -1 + probability_outcome ;
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_sub_overflow() {
        let probability_outcome_one = ProbabilityOutcome {value: i32::MAX,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let probability_outcome_two = ProbabilityOutcome {value: -1,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let _ = probability_outcome_one - probability_outcome_two;
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_sub_underflow() {
        let probability_outcome_one = ProbabilityOutcome {value: i32::MIN,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let probability_outcome_two = ProbabilityOutcome {value: 1,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let _ = probability_outcome_one - probability_outcome_two;
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_sub_i32_overflow() {
        let probability_outcome = ProbabilityOutcome {value: i32::MAX,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let _ = probability_outcome - -1;
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_sub_i32_underflow() {
        let probability_outcome = ProbabilityOutcome {value: i32::MIN,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let _ = probability_outcome - 1;
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_i32_sub_overflow() {
        let probability_outcome = ProbabilityOutcome {value: -1,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let _ = i32::MAX - probability_outcome;
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_132_sub_underflow() {
        let probability_outcome = ProbabilityOutcome {value: 1,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let _ = i32::MIN - probability_outcome;
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_mul_overflow() {
        let probability_outcome_one = ProbabilityOutcome {value: i32::MAX,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let probability_outcome_two = ProbabilityOutcome {value: 2,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let _ = probability_outcome_one * probability_outcome_two;
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_mul_underflow() {
        let probability_outcome_one = ProbabilityOutcome {value: i32::MIN,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let probability_outcome_two = ProbabilityOutcome {value: -2,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let _ = probability_outcome_one * probability_outcome_two;
    }
    
    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_mul_i32_overflow() {
        let probability_outcome = ProbabilityOutcome {value: i32::MAX,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let _ = probability_outcome * 2;
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_mul_i32_underflow() {
        let probability_outcome = ProbabilityOutcome {value: i32::MIN,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let _ = probability_outcome * -2;
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_i32_mul_overflow() {
        let probability_outcome = ProbabilityOutcome {value: 2,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let _ = i32::MAX * probability_outcome;
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_132_mul_underflow() {
        let probability_outcome = ProbabilityOutcome {value: -2,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let _ = i32::MIN * probability_outcome;
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_div_by_zero() {
        let probability_outcome_one = ProbabilityOutcome {value: i32::MAX,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let probability_outcome_two = ProbabilityOutcome {value: 0,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let _ = probability_outcome_one / probability_outcome_two;
    }
    
    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_div_i32_by_zero() {
        let probability_outcome = ProbabilityOutcome {value: i32::MAX,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let _ = probability_outcome / 0;
    }
    
    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_i32_div_by_zero() {
        let probability_outcome = ProbabilityOutcome {value: 0,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let _ = i32::MAX / probability_outcome;
    }
        
    #[test]
    #[should_panic(expected = "attempt to calculate the remainder with a divisor of zero")]
    fn test_rem_by_zero() {
        let probability_outcome_one = ProbabilityOutcome {value: i32::MAX,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let probability_outcome_two = ProbabilityOutcome {value: 0,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let _ = probability_outcome_one % probability_outcome_two;
    }
       
    #[test]
    #[should_panic(expected = "attempt to calculate the remainder with a divisor of zero")]
    fn test_rem_i32_by_zero() {
        let probability_outcome = ProbabilityOutcome {value: i32::MAX,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let _ = probability_outcome % 0;
    }
    
    #[test]
    #[should_panic(expected = "attempt to calculate the remainder with a divisor of zero")]
    fn test_i32_rem_by_zero() {
        let probability_outcome = ProbabilityOutcome {value: 0,  constraint_map: ConstraintMap {map: HashMap::new()}};
        let _ = i32::MAX % probability_outcome;
    }

}
