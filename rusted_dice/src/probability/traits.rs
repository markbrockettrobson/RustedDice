use crate::probability::BinaryOperation;

pub trait Combine {
    fn combine(&self, other: Self, binary_operation: BinaryOperation) -> Self;
    fn combinei32(&self, other: i32, binary_operation: BinaryOperation) -> Self;
    fn i32combine(&self, other: i32, binary_operation: BinaryOperation) -> Self;
}
