use crate::ValueType;

/// A type representing a function taking two [ValueType], [ValueType] returning [ValueType].
pub type BinaryOperation = fn(ValueType, ValueType) -> ValueType;
