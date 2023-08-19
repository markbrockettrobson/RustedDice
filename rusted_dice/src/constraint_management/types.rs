use std::collections::{HashMap, HashSet};

use crate::constraint_management::Constraint;
use crate::ValueType;

/// A type representing a unique identifier for a [Constraint].
pub type ConstraintIdType = u16;

/// A type representing a [HashSet] of [ValueType].
pub type ValueTypeSet = HashSet<ValueType>;

/// A type representing a [HashMap], [ConstraintIdType] to their corresponding [Constraint] objects.
pub type ConstraintIdToConstraintHashMap = HashMap<ConstraintIdType, Constraint>;

/// A type representing a [HashMap], [ConstraintIdType] to the associated [ValueType].
pub type IdToValueMap = HashMap<ConstraintIdType, ValueType>;
