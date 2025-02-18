//! field_vector_base.rs
//!
//! Implements the basic primitives needed to support a library's data type
use crate::data_types::MyPrimitiveDataTypes;

/// Data Format supported by this data engine
pub enum Format {
    ARROW,
}

/// Basic interface for an array builder.
pub trait MyArrayBuilderInterface {
    /// The type of builder
    const BUILDER_TYPE: Format;
    /// data type, this should 
    const DATA_TYPE: MyPrimitiveDataTypes;
    /// The type that will be returned once the builder is finished
    type ReturnTypeAtBuild;
    /// the value type to be provided to fill up the array
    type ValueType;

    /// Create a new builder of certain capacity
    fn with_capacity(initial_capacity: usize) -> Self;
    /// Append a new element to the array
    fn append(&mut self, value: Self::ValueType);
    /// Construct the array
    fn finish(&mut self) -> MyArray<Self::ReturnTypeAtBuild>;
}

// actual data when done building the Array buffer
pub struct MyArray<T> {
    data: T,
}

impl<T> MyArray<T> {
    /// Construct a new Array from native data
    pub fn from_native(data: T) -> Self {
        Self { data }
    }

    /// Get the native raw buffer
    pub fn get_native_array(&self) -> &T {
        &self.data
    }
}
