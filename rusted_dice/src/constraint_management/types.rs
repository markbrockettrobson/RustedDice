use std::collections::HashMap;

use crate::constraint_management::Constraint;

pub type ConstraintIdType = u16;
pub type ConstraintValueType = i32;

pub type IdToConstraintMap = HashMap<ConstraintIdType, Constraint>;
pub type IdToValueMap = HashMap<ConstraintIdType, ConstraintValueType>;
