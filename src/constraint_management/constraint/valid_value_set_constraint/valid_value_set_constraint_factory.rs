use std::{collections::HashSet, hash::Hash};

use core::fmt::Debug;

use crate::constraint_management::{ConstraintIdType, ValidValueSetConstraint};

#[allow(dead_code)]
impl<T> ValidValueSetConstraint<T>
where
    T: Eq + Hash + Debug + Ord,
{
    /// Creates a new empty [ValidValueSetConstraint] with the given ID.
    ///
    /// An empty [ValidValueSetConstraint] has no valid values associated with it.
    ///
    /// # Arguments
    ///
    /// * `id` - The [ConstraintIdType] for the [ValidValueSetConstraint].
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::constraint_management::ValidValueSetConstraint;
    /// let constraint = ValidValueSetConstraint::<u32>::new_empty_constraint(1);
    /// ```
    pub fn new_empty_constraint(id: ConstraintIdType) -> ValidValueSetConstraint<T> {
        let valid_values: HashSet<T> = HashSet::new();
        ValidValueSetConstraint { id, valid_values }
    }

    /// Creates a new [ValidValueSetConstraint] with a single valid value.
    ///
    /// The [ValidValueSetConstraint] allows only a single value to be valid.
    ///
    /// # Arguments
    ///
    /// * `id` - The [ConstraintIdType] for the [ValidValueSetConstraint].
    /// * `value` - The T for the [ValidValueSetConstraint].
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::constraint_management::ValidValueSetConstraint;
    /// let constraint = ValidValueSetConstraint::new_single_valid_value_constraint(2, "apple");
    /// ```
    pub fn new_single_valid_value_constraint(
        id: ConstraintIdType,
        value: T,
    ) -> ValidValueSetConstraint<T> {
        let valid_values: HashSet<T> = vec![value].into_iter().collect();
        ValidValueSetConstraint { id, valid_values }
    }

    /// Creates a new [ValidValueSetConstraint] with multiple valid values.
    ///
    /// The [ValidValueSetConstraint] allows multiple values to be valid.
    ///
    /// # Arguments
    ///
    /// * `id` - The [ConstraintIdType] for the [ValidValueSetConstraint].
    /// * `values` - An iterator over values of T for the [ValidValueSetConstraint].
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusted_dice::constraint_management::ValidValueSetConstraint;
    /// let constraint = ValidValueSetConstraint::new_many_item_constraint(3, vec![1, 2]);
    /// ```
    pub fn new_many_item_constraint(
        id: ConstraintIdType,
        values: impl IntoIterator<Item = T>,
    ) -> ValidValueSetConstraint<T> {
        let valid_values: HashSet<T> = values.into_iter().collect();
        ValidValueSetConstraint { id, valid_values }
    }
}
