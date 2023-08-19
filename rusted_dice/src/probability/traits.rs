use crate::{probability::BinaryOperation, ValueType};

/// A trait for objects that can perform a [BinaryOperation] with another instance of the same type or [ValueType].
pub trait Combine {
    /// Combine this instance with another instance using the specified [BinaryOperation].
    ///
    /// # Arguments
    ///
    /// * `other` - The self type to check preform the [BinaryOperation] with.
    /// * 'binary_operation' - the [BinaryOperation] function.
    ///
    /// # Returns
    ///
    /// Returns the self type result of the [BinaryOperation] function.
    fn combine(&self, other: Self, binary_operation: BinaryOperation) -> Self;

    /// Combine this instance with a [ValueType] using the specified [BinaryOperation].
    /// in the order: self [BinaryOperation] [ValueType]
    ///
    /// # Arguments
    ///
    /// * `other` - The [ValueType] to check preform the [BinaryOperation] with.
    /// * 'binary_operation' - the [BinaryOperation] function.
    ///
    /// # Returns
    ///
    /// Returns the self type result of the [BinaryOperation] function.
    fn combine_value_type(&self, other: ValueType, binary_operation: BinaryOperation) -> Self;

    /// Combine this instance with a [ValueType] using the specified [BinaryOperation].
    /// in the order: [ValueType] [BinaryOperation] self
    ///
    /// # Arguments
    ///
    /// * `other` - The [ValueType] to check preform the [BinaryOperation] with.
    /// * 'binary_operation' - the [BinaryOperation] function.
    ///
    /// # Returns
    ///
    /// Returns the self type result of the [BinaryOperation] function.
    fn value_type_combine(&self, other: ValueType, binary_operation: BinaryOperation) -> Self;
}
