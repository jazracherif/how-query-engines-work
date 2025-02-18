//! data_types.rs
//!
//! Contains all the data types supported by this engine

pub enum MyPrimitiveDataTypes {
    // signed
    Int8,
    Int16,
    Int32,
    Int64,
    // unsigned
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    // single and double precision points
    Float,
    Double,
}

//TODO: add additional types
// Boolean,
// String,

// this trait can be used by end user to request operation on certain types
pub trait MyDataTypePrimitives {
    const DATA_TYPE: MyPrimitiveDataTypes;
}

// This macro creates new base data types.
#[macro_export]
macro_rules! make_base_type {
    ($name:ident, $data_type:expr,  $native_type:ty,  $doc_string: literal) => {
        #[derive(Debug)]
        #[doc = $doc_string]
        pub struct $name {}

        impl MyDataTypePrimitives for $name {
            const DATA_TYPE: MyPrimitiveDataTypes = $data_type;
        }
    };
}
