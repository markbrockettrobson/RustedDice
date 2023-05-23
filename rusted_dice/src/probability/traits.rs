use crate::{probability::BinaryOperation, ValueType};

pub trait Combine {
    fn combine(&self, other: Self, binary_operation: BinaryOperation) -> Self;
    fn combine_value_type(&self, other: ValueType, binary_operation: BinaryOperation) -> Self;
    fn value_type_combine(&self, other: ValueType, binary_operation: BinaryOperation) -> Self;
}
