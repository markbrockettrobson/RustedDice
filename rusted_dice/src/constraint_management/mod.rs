pub(crate) mod constraint;
pub(crate) mod constraint_map;
pub(crate) mod traits;
pub(crate) mod types;

pub use constraint::Constraint;
pub use constraint::ConstraintFactory;

pub use constraint_map::ConstraintMap;
pub use constraint_map::ConstraintMapFactory;

pub use traits::AreConstraintsCompiledWith;
pub use traits::IsConstraintCompiledWith;
pub use traits::IsTheoreticallyPossible;

pub use types::ConstraintIdType;
pub use types::ConstraintValueType;
pub use types::IdToConstraintMap;
pub use types::IdToValueMap;
