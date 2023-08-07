/// The type of a possible state in a probability distribution.
pub type ValueType = i32;

/// NB!
/// Intended for use in tests that need values safly inside the bounds of valueType.
///
/// see [ValueType], [UnsignedSmallValueType]
pub type SmallValueType = i16;

/// NB!
/// Intended for use in tests that need unsigned values safly inside the bounds of valueType.
///
/// see [ValueType], [SmallValueType]
pub type UnsignedSmallValueType = u16;
