use crate::constraint_management::IdToValueMap;
use crate::ValueType;
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
pub trait IsConstraintCompiledWith {
    // Checks whether the constraint represented by this object is complied with the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The [ValueType] to check compliance against.
    ///
    /// # Returns
    ///
    /// Returns `true` if the constraint is complied with the given value, `false` otherwise.
    fn is_compliant_with(&self, value: ValueType) -> bool;
}

/// A trait for objects that can determine whether a set of constraints are complied with.
pub trait AreConstraintsCompiledWith {
    /// Checks whether the set of Constraints represented by this object are complied with the given [IdToValueMap].
    ///
    /// # Arguments
    ///
    /// * `value_map` - [IdToValueMap] to check compliance against.
    ///
    /// # Returns
    ///
    /// Returns `true` if all Constraints are complied with the given value map, `false` otherwise.
    fn is_compliant_with(&self, value_map: IdToValueMap) -> bool;
}
