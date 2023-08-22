use std::collections::BTreeMap;

use crate::{CountType, ValueType};

use super::ProbabilityOutcome;

/// A type representing a function taking two [ValueType], [ValueType] returning [ValueType].
pub type BinaryOperation = fn(ValueType, ValueType) -> ValueType;

/// A type representing a [BTreeMap] mapping [ProbabilityOutcome] to a count [CountType].
pub type OutcomeToCountMap = BTreeMap<ProbabilityOutcome, CountType>;
