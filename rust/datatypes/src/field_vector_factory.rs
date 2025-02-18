use crate::field_vector_arrow::{MyArrowDataTypePrimitives, MyPrimitiveArrowArrayBuilder};
use crate::field_vector_base::MyArrayBuilderInterface;

/// This is the Array builder interface
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
