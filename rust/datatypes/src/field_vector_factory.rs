//! field_vector_factor.rs
//! 
//! This factory is the entrypoint for creating various kinds of array, and for different
//! formats.

use crate::field_vector_arrow::{MyArrowDataTypePrimitives, MyPrimitiveArrowArrayBuilder};
use crate::field_vector_base::MyArrayBuilderInterface;

pub struct ArrayBuilderFactory {}
impl ArrayBuilderFactory {
    /// Arrow Methods
    pub fn primitive_array<T>(initial_capacity: usize) -> MyPrimitiveArrowArrayBuilder<T>
    where
        T: MyArrowDataTypePrimitives,
    {
        MyPrimitiveArrowArrayBuilder::<T>::with_capacity(initial_capacity)
    }
}
