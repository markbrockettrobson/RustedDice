/// The type of a possible state in a probability distribution.
pub type ValueType = i32;

/// NB!
/// Intended for use in tests that need values safely inside the bounds of valueType.
///
/// see [ValueType], [UnsignedSmallValueType]
pub type SmallValueType = i16;

/// NB!
/// Intended for use in tests that need unsigned values safely inside the bounds of valueType.
///
/// see [ValueType], [SmallValueType]
pub type UnsignedSmallValueType = u16;

/// The type of a count of possible ways to obtain a state in a probability distribution.
pub type CountType = u64;
