use std::collections::{HashMap, HashSet};

use crate::ValueType;

/// A type representing a unique identifier for a [Constraint].
pub type ConstraintIdType = u16;

/// A type representing a [HashSet] of [ValueType].
pub type ValueTypeSet = HashSet<ValueType>;

/// A type representing a [HashMap], [ConstraintIdType] to the associated [ValueType].
pub type IdToValueMap = HashMap<ConstraintIdType, ValueType>;

pub type ConstraintIdToConstraintHashMap = HashMap<ConstraintIdType, ValueTypeSet>;