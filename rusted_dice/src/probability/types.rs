use crate::ValueType;

pub type BinaryOperation = fn(ValueType, ValueType) -> ValueType;
