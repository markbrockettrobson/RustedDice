use std::collections::HashMap;
use std::collections::HashSet;

use crate::constraint_management::ConstraintIdType;
/// A trait for objects that can determine whether a condition or collection of conditions are theoretically possible.
pub trait IsTheoreticallyPossible {
    /// Checks whether the condition represented by this object is theoretically possible.
    ///
    /// # Returns
    ///
    /// Returns `true` if the condition or collection of conditions are theoretically possible, `false` otherwise.
    fn is_theoretically_possible(&self) -> bool;
}

/// A trait for objects that can determine whether a certain constraint is complied with.
pub trait IsConstraintCompiledWith<T> {
    // Checks whether the constraint represented by this object is complied with the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - Any type to check compliance against.
    ///
    /// # Returns
    ///
    /// Returns `true` if the constraint is complied with the given value, `false` otherwise.
    /// Note if the value is not of the expected type, the function will panic.
    fn is_compliant_with(&self, value: T) -> bool;
}

/// A trait for objects that can determine whether a set of constraints are complied with.
pub trait AreConstraintsCompiledWith<T> {
    /// Checks whether the set of Constraints represented by this object are complied with the given [HashMap<ConstraintIdType, T>].
    ///
    /// # Arguments
    ///
    /// * `value_map` - HashMap<ConstraintIdType, T> to check compliance against.
    ///
    /// # Returns
    ///
    /// Returns `true` if all Constraints are complied with the given value map, `false` otherwise.
    fn is_compliant_with(&self, value_map: HashMap<ConstraintIdType, T>) -> bool;
}

/// A trait for objects with a set of valid values.
pub trait GetValidValues<T> {
    /// Gets the set of valid values.
    /// # Returns
    /// Returns a [&HashSet<T>] of valid values.

    fn get_valid_values(&self) -> &HashSet<T>;

}

pub trait GetId {
    /// Gets the id of the constraint.
    /// # Returns
    /// Returns a [ConstraintIdType] of the constraint.
    fn get_id(&self) -> ConstraintIdType;
}