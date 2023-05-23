use std::collections::{HashMap, HashSet};

use crate::constraint_management::Constraint;
use crate::ValueType;

pub type ConstraintIdType = u16;
pub type ValueTypeSet = HashSet<ValueType>;
pub type IdToConstraintMap = HashMap<ConstraintIdType, Constraint>;
pub type IdToValueMap = HashMap<ConstraintIdType, ValueType>;
