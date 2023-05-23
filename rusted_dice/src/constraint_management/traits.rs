use crate::constraint_management::IdToValueMap;
use crate::ValueType;

pub trait IsTheoreticallyPossible {
    fn is_theoretically_possible(&self) -> bool;
}

pub trait IsConstraintCompiledWith {
    fn is_compliant_with(&self, value: ValueType) -> bool;
}

pub trait AreConstraintsCompiledWith {
    fn is_compliant_with(&self, value_map: IdToValueMap) -> bool;
}
