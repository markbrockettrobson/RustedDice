use std::cmp::{Ord, Eq, PartialOrd};
use std::ops::{Add, Sub, Mul, Div, BitOr, BitXor, BitAnd, Not, Neg, Rem};

#[derive(Copy, Clone, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct ProbabilityOutcome {
    pub value: i32,
}

impl Add for ProbabilityOutcome {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { value: self.value + other.value }
    }
}

impl Add<i32> for ProbabilityOutcome {
    type Output = Self;

    fn add(self, other: i32) -> Self {
        Self { value: self.value + other }
    }
}

impl Add<ProbabilityOutcome> for i32 {
    type Output = ProbabilityOutcome;

    fn add(self, other: ProbabilityOutcome) -> ProbabilityOutcome {
        ProbabilityOutcome { value: self + other.value }
    }
}

impl Sub for ProbabilityOutcome {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { value: self.value - other.value }
    }
}

impl Sub<i32> for ProbabilityOutcome {
    type Output = Self;

    fn sub(self, other: i32) -> Self {
        Self { value: self.value - other }
    }
}

impl Sub<ProbabilityOutcome> for i32 {
    type Output = ProbabilityOutcome;

    fn sub(self, other: ProbabilityOutcome) -> ProbabilityOutcome {
        ProbabilityOutcome { value: self - other.value }
    }
}

impl Mul for ProbabilityOutcome {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self { value: self.value * other.value }
    }
}

impl Div for ProbabilityOutcome {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self { value: self.value / other.value }
    }
}

impl Rem for ProbabilityOutcome {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Self { value: self.value % other.value }
    }
}

impl BitOr for ProbabilityOutcome {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Self { value: self.value | other.value }
    }
}

impl BitXor for ProbabilityOutcome {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        Self { value: self.value ^ other.value }
    }
}

impl BitAnd for ProbabilityOutcome {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        Self { value: self.value & other.value }
    }
}

impl Not for ProbabilityOutcome {
    type Output = Self;

    fn not(self) -> Self {
        Self { value: !self.value }
    }
}

impl Neg for ProbabilityOutcome {
    type Output = Self;

    fn neg(self) -> Self {
        Self { value: -self.value }
    }
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

        #[test]
        fn test_add(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) + i32::from(value_two);
            let probability_outcome_one = ProbabilityOutcome {value: i32::from(value_one)};
            let probability_outcome_two = ProbabilityOutcome {value: i32::from(value_two)};
            let result = probability_outcome_one + probability_outcome_two;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_add_i32(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) + i32::from(value_two);
            let probability_outcome = ProbabilityOutcome {value: i32::from(value_one)};
            let result = probability_outcome + i32::from(value_two);
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_i32_add(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) + i32::from(value_two);
            let probability_outcome = ProbabilityOutcome {value: i32::from(value_one)};
            let result = i32::from(value_two) + probability_outcome ;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_sub(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) - i32::from(value_two);
            let probability_outcome_one = ProbabilityOutcome {value: i32::from(value_one)};
            let probability_outcome_two = ProbabilityOutcome {value: i32::from(value_two)};
            let result = probability_outcome_one - probability_outcome_two;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_sub_i32(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) - i32::from(value_two);
            let probability_outcome = ProbabilityOutcome {value: i32::from(value_one)};
            let result = probability_outcome - i32::from(value_two);
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_i32_sub(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) - i32::from(value_two);
            let probability_outcome = ProbabilityOutcome {value: i32::from(value_two)};
            let result = i32::from(value_one) - probability_outcome;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_mul(value_one: i16, value_two: i16) {
            let expected_value = i32::from(value_one) * i32::from(value_two);
            let probability_outcome_one = ProbabilityOutcome {value: i32::from(value_one)};
            let probability_outcome_two = ProbabilityOutcome {value: i32::from(value_two)};
            let result = probability_outcome_one * probability_outcome_two;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_div(value_one: i32, value_two: i16) {
            prop_assume!(value_two != 0);
            let expected_value = value_one / i32::from(value_two);
            let probability_outcome_one = ProbabilityOutcome {value: value_one};
            let probability_outcome_two = ProbabilityOutcome {value: i32::from(value_two)};
            let result = probability_outcome_one / probability_outcome_two;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_rem(value_one: i32, value_two: i16) {
            prop_assume!(value_two != 0);
            let expected_value = value_one % i32::from(value_two);
            let probability_outcome_one = ProbabilityOutcome {value: value_one};
            let probability_outcome_two = ProbabilityOutcome {value: i32::from(value_two)};
            let result = probability_outcome_one % probability_outcome_two;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_bitor(value_one: i32, value_two: i32) {
            let expected_value = value_one | value_two;
            let probability_outcome_one = ProbabilityOutcome {value: value_one};
            let probability_outcome_two = ProbabilityOutcome {value: value_two};
            let result = probability_outcome_one | probability_outcome_two;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_bitxor(value_one: i32, value_two: i32) {
            let expected_value = value_one ^ value_two;
            let probability_outcome_one = ProbabilityOutcome {value: value_one};
            let probability_outcome_two = ProbabilityOutcome {value: value_two};
            let result = probability_outcome_one ^ probability_outcome_two;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_bitand(value_one: i32, value_two: i32) {
            let expected_value = value_one & value_two;
            let probability_outcome_one = ProbabilityOutcome {value: value_one};
            let probability_outcome_two = ProbabilityOutcome {value: value_two};
            let result = probability_outcome_one & probability_outcome_two;
            assert_eq!(result.value, expected_value);
        }
        
        #[test]
        fn test_not(value_one: i32) {
            let expected_value = !value_one;
            let probability_outcome = ProbabilityOutcome {value: value_one};
            let result = !probability_outcome;
            assert_eq!(result.value, expected_value);
        }

        #[test]
        fn test_neg(value_one: i32) {
            let expected_value = -value_one;
            let probability_outcome = ProbabilityOutcome {value: value_one};
            let result = -probability_outcome;
            assert_eq!(result.value, expected_value);
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

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_add_overflow() {
        let probability_outcome_one = ProbabilityOutcome {value: i32::MAX};
        let probability_outcome_two = ProbabilityOutcome {value: 1};
        let _ = probability_outcome_one + probability_outcome_two;
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_add_underflow() {
        let probability_outcome_one = ProbabilityOutcome {value: i32::MIN};
        let probability_outcome_two = ProbabilityOutcome {value: -1};
        let _ = probability_outcome_one + probability_outcome_two;
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_add_i32_overflow() {
        let probability_outcome = ProbabilityOutcome {value: i32::MAX};
        let _ = probability_outcome + 1;
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_add_i32_underflow() {
        let probability_outcome = ProbabilityOutcome {value: i32::MIN};
        let _ = probability_outcome + -1;
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_i32_add_overflow() {
        let probability_outcome = ProbabilityOutcome {value: i32::MAX};
        let _ = 1 + probability_outcome;
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_i32_add_underflow() {
        let probability_outcome = ProbabilityOutcome {value: i32::MIN};
        let _ = -1 + probability_outcome ;
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_sub_overflow() {
        let probability_outcome_one = ProbabilityOutcome {value: i32::MAX};
        let probability_outcome_two = ProbabilityOutcome {value: -1};
        let _ = probability_outcome_one - probability_outcome_two;
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_sub_underflow() {
        let probability_outcome_one = ProbabilityOutcome {value: i32::MIN};
        let probability_outcome_two = ProbabilityOutcome {value: 1};
        let _ = probability_outcome_one - probability_outcome_two;
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_sub_i32_overflow() {
        let probability_outcome = ProbabilityOutcome {value: i32::MAX};
        let _ = probability_outcome - -1;
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_sub_i32_underflow() {
        let probability_outcome = ProbabilityOutcome {value: i32::MIN};
        let _ = probability_outcome - 1;
    }


    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_i32_sub_overflow() {
        let probability_outcome = ProbabilityOutcome {value: -1};
        let _ = i32::MAX - probability_outcome;
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_132_sub_underflow() {
        let probability_outcome = ProbabilityOutcome {value: 1};
        let _ = i32::MIN - probability_outcome;
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_mul_overflow() {
        let probability_outcome_one = ProbabilityOutcome {value: i32::MAX};
        let probability_outcome_two = ProbabilityOutcome {value: 2};
        let _ = probability_outcome_one * probability_outcome_two;
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn test_mul_underflow() {
        let probability_outcome_one = ProbabilityOutcome {value: i32::MIN};
        let probability_outcome_two = ProbabilityOutcome {value: -2};
        let _ = probability_outcome_one * probability_outcome_two;
    }
    
    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_div_by_zero() {
        let probability_outcome_one = ProbabilityOutcome {value: i32::MAX};
        let probability_outcome_two = ProbabilityOutcome {value: 0};
        let _ = probability_outcome_one / probability_outcome_two;
    }
        
    #[test]
    #[should_panic(expected = "attempt to calculate the remainder with a divisor of zero")]
    fn test_rem_by_zero() {
        let probability_outcome_one = ProbabilityOutcome {value: i32::MAX};
        let probability_outcome_two = ProbabilityOutcome {value: 0};
        let _ = probability_outcome_one % probability_outcome_two;
    }
}
