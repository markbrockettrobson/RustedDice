use std::collections::HashSet;

use proptest::prelude::*;

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub(crate) enum TestValueTypeEnum {
    Int8(i8),
    Uint8(u8),
    Int16(i16),
    Uint16(u16),
    Int32(i32),
    Uint32(u32),
    Int64(i64),
    Uint64(u64),
    Int128(i128),
    Uint128(u128),
    Char(char),
    String(String),
    Bool(bool),
    VecInt32(Vec<i32>),
    VecUint32(Vec<u32>),
    VecString(Vec<String>),
    VecChar(Vec<char>),
    VecBool(Vec<bool>),
}

#[allow(dead_code)]
pub(crate) fn test_value_strategy() -> impl Strategy<Value = TestValueTypeEnum> {
    prop_oneof![
        any::<i8>().prop_map(TestValueTypeEnum::Int8),
        any::<u8>().prop_map(TestValueTypeEnum::Uint8),
        any::<i16>().prop_map(TestValueTypeEnum::Int16),
        any::<u16>().prop_map(TestValueTypeEnum::Uint16),
        any::<i32>().prop_map(TestValueTypeEnum::Int32),
        any::<u32>().prop_map(TestValueTypeEnum::Uint32),
        any::<i64>().prop_map(TestValueTypeEnum::Int64),
        any::<u64>().prop_map(TestValueTypeEnum::Uint64),
        any::<i128>().prop_map(TestValueTypeEnum::Int128),
        any::<u128>().prop_map(TestValueTypeEnum::Uint128),
        any::<char>().prop_map(TestValueTypeEnum::Char),
        any::<String>().prop_map(TestValueTypeEnum::String),
        any::<bool>().prop_map(TestValueTypeEnum::Bool),
        any::<Vec<i32>>().prop_map(TestValueTypeEnum::VecInt32),
        any::<Vec<u32>>().prop_map(TestValueTypeEnum::VecUint32),
        any::<Vec<String>>().prop_map(TestValueTypeEnum::VecString),
        any::<Vec<char>>().prop_map(TestValueTypeEnum::VecChar),
        any::<Vec<bool>>().prop_map(TestValueTypeEnum::VecBool),
    ]
}

#[allow(dead_code)]
pub(crate) fn test_vec_value_strategy(
    min_count: usize,
    max_count: usize,
) -> impl Strategy<Value = Vec<TestValueTypeEnum>> {
    prop_oneof![
        prop::collection::vec(any::<i8>(), min_count..max_count)
            .prop_map(|v| v.into_iter().map(TestValueTypeEnum::Int8).collect()),
        prop::collection::vec(any::<u8>(), min_count..max_count)
            .prop_map(|v| v.into_iter().map(TestValueTypeEnum::Uint8).collect()),
        prop::collection::vec(any::<i16>(), min_count..max_count)
            .prop_map(|v| v.into_iter().map(TestValueTypeEnum::Int16).collect()),
        prop::collection::vec(any::<u16>(), min_count..max_count)
            .prop_map(|v| v.into_iter().map(TestValueTypeEnum::Uint16).collect()),
        prop::collection::vec(any::<i32>(), min_count..max_count)
            .prop_map(|v| v.into_iter().map(TestValueTypeEnum::Int32).collect()),
        prop::collection::vec(any::<u32>(), min_count..max_count)
            .prop_map(|v| v.into_iter().map(TestValueTypeEnum::Uint32).collect()),
        prop::collection::vec(any::<i64>(), min_count..max_count)
            .prop_map(|v| v.into_iter().map(TestValueTypeEnum::Int64).collect()),
        prop::collection::vec(any::<u64>(), min_count..max_count)
            .prop_map(|v| v.into_iter().map(TestValueTypeEnum::Uint64).collect()),
        prop::collection::vec(any::<i128>(), min_count..max_count)
            .prop_map(|v| v.into_iter().map(TestValueTypeEnum::Int128).collect()),
        prop::collection::vec(any::<u128>(), min_count..max_count)
            .prop_map(|v| v.into_iter().map(TestValueTypeEnum::Uint128).collect()),
        prop::collection::vec(any::<char>(), min_count..max_count)
            .prop_map(|v| v.into_iter().map(TestValueTypeEnum::Char).collect()),
        prop::collection::vec(any::<String>(), min_count..max_count)
            .prop_map(|v| v.into_iter().map(TestValueTypeEnum::String).collect()),
        prop::collection::vec(any::<bool>(), min_count..max_count)
            .prop_map(|v| v.into_iter().map(TestValueTypeEnum::Bool).collect()),
        prop::collection::vec(
            prop::collection::vec(any::<i32>(), min_count..max_count),
            min_count..max_count
        )
        .prop_map(|v| v.into_iter().map(TestValueTypeEnum::VecInt32).collect()),
        prop::collection::vec(
            prop::collection::vec(any::<u32>(), min_count..max_count),
            min_count..max_count
        )
        .prop_map(|v| v.into_iter().map(TestValueTypeEnum::VecUint32).collect()),
        prop::collection::vec(
            prop::collection::vec(any::<String>(), min_count..max_count),
            min_count..max_count
        )
        .prop_map(|v| v.into_iter().map(TestValueTypeEnum::VecString).collect()),
        prop::collection::vec(
            prop::collection::vec(any::<char>(), min_count..max_count),
            min_count..max_count
        )
        .prop_map(|v| v.into_iter().map(TestValueTypeEnum::VecChar).collect()),
        prop::collection::vec(
            prop::collection::vec(any::<bool>(), min_count..max_count),
            min_count..max_count
        )
        .prop_map(|v| v.into_iter().map(TestValueTypeEnum::VecBool).collect()),
    ]
}

#[allow(dead_code)]
pub(crate) fn test_hash_set_value_strategy(
    min_count: usize,
    max_count: usize,
) -> impl Strategy<Value = HashSet<TestValueTypeEnum>> {
    prop_oneof![
        prop::collection::hash_set(any::<i8>(), min_count..max_count)
            .prop_map(|v| v.into_iter().map(TestValueTypeEnum::Int8).collect()),
        prop::collection::hash_set(any::<u8>(), min_count..max_count)
            .prop_map(|v| v.into_iter().map(TestValueTypeEnum::Uint8).collect()),
        prop::collection::hash_set(any::<i16>(), min_count..max_count)
            .prop_map(|v| v.into_iter().map(TestValueTypeEnum::Int16).collect()),
        prop::collection::hash_set(any::<u16>(), min_count..max_count)
            .prop_map(|v| v.into_iter().map(TestValueTypeEnum::Uint16).collect()),
        prop::collection::hash_set(any::<i32>(), min_count..max_count)
            .prop_map(|v| v.into_iter().map(TestValueTypeEnum::Int32).collect()),
        prop::collection::hash_set(any::<u32>(), min_count..max_count)
            .prop_map(|v| v.into_iter().map(TestValueTypeEnum::Uint32).collect()),
        prop::collection::hash_set(any::<i64>(), min_count..max_count)
            .prop_map(|v| v.into_iter().map(TestValueTypeEnum::Int64).collect()),
        prop::collection::hash_set(any::<u64>(), min_count..max_count)
            .prop_map(|v| v.into_iter().map(TestValueTypeEnum::Uint64).collect()),
        prop::collection::hash_set(any::<i128>(), min_count..max_count)
            .prop_map(|v| v.into_iter().map(TestValueTypeEnum::Int128).collect()),
        prop::collection::hash_set(any::<u128>(), min_count..max_count)
            .prop_map(|v| v.into_iter().map(TestValueTypeEnum::Uint128).collect()),
        prop::collection::hash_set(any::<char>(), min_count..max_count)
            .prop_map(|v| v.into_iter().map(TestValueTypeEnum::Char).collect()),
        prop::collection::hash_set(any::<String>(), min_count..max_count)
            .prop_map(|v| v.into_iter().map(TestValueTypeEnum::String).collect()),
        prop::collection::hash_set(any::<bool>(), min_count..max_count)
            .prop_map(|v| v.into_iter().map(TestValueTypeEnum::Bool).collect()),
        prop::collection::hash_set(
            prop::collection::vec(any::<i32>(), min_count..max_count),
            min_count..max_count
        )
        .prop_map(|v| v.into_iter().map(TestValueTypeEnum::VecInt32).collect()),
        prop::collection::hash_set(
            prop::collection::vec(any::<u32>(), min_count..max_count),
            min_count..max_count
        )
        .prop_map(|v| v.into_iter().map(TestValueTypeEnum::VecUint32).collect()),
        prop::collection::hash_set(
            prop::collection::vec(any::<String>(), min_count..max_count),
            min_count..max_count
        )
        .prop_map(|v| v.into_iter().map(TestValueTypeEnum::VecString).collect()),
        prop::collection::hash_set(
            prop::collection::vec(any::<char>(), min_count..max_count),
            min_count..max_count
        )
        .prop_map(|v| v.into_iter().map(TestValueTypeEnum::VecChar).collect()),
        prop::collection::hash_set(
            prop::collection::vec(any::<bool>(), min_count..max_count),
            min_count..max_count
        )
        .prop_map(|v| v.into_iter().map(TestValueTypeEnum::VecBool).collect()),
    ]
}
