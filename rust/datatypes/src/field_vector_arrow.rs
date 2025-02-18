use arrow;
use arrow::datatypes::ArrowPrimitiveType;

use crate::data_types::{MyDataTypePrimitives, MyPrimitiveDataTypes};
use crate::field_vector_base::{Format, MyArray, MyArrayBuilderInterface};
use crate::make_base_type;

pub trait MyArrowDataTypePrimitives: MyDataTypePrimitives {
    type MyNative: ArrowPrimitiveType; // this needs to be the target library type
}

// Appends arrow specific native types to the basic data type structure
macro_rules! make_arrow_type {
    ($name:ident, $data_ty:expr,  $native_type:ty,  $doc_string: literal) => {
        make_base_type!($name, $data_ty, $native_type, $doc_string);
        impl MyArrowDataTypePrimitives for $name {
            type MyNative = $native_type;
        }
    };
}

// Create the arrow types that map to the Data Engine's data types
make_arrow_type!(
    MyArrowInt8Type,
    MyPrimitiveDataTypes::Int8,
    arrow::datatypes::Int8Type,
    "A signed 8-bit integer type."
);
make_arrow_type!(
    MyArrowInt16Type,
    MyPrimitiveDataTypes::Int16,
    arrow::datatypes::Int16Type,
    "A signed 16-bit integer type."
);
make_arrow_type!(
    MyArrowInt32Type,
    MyPrimitiveDataTypes::Int32,
    arrow::datatypes::Int32Type,
    "A signed 32-bit integer type."
);
make_arrow_type!(
    MyArrowInt64Type,
    MyPrimitiveDataTypes::Int64,
    arrow::datatypes::Int64Type,
    "A signed 64-bit integer type."
);

make_arrow_type!(
    MyArrowUInt8Type,
    MyPrimitiveDataTypes::UInt8,
    arrow::datatypes::UInt8Type,
    "An unsigned 8-bit integer type."
);
make_arrow_type!(
    MyArrowUInt16Type,
    MyPrimitiveDataTypes::UInt16,
    arrow::datatypes::UInt16Type,
    "An unsigned 16-bit integer type."
);
make_arrow_type!(
    MyArrowUInt32Type,
    MyPrimitiveDataTypes::UInt32,
    arrow::datatypes::UInt32Type,
    "An unsigned 32-bit integer type."
);
make_arrow_type!(
    MyArrowUInt64Type,
    MyPrimitiveDataTypes::UInt64,
    arrow::datatypes::UInt64Type,
    "An unsigned 64-bit integer type."
);

make_arrow_type!(
    MyArrowFloatType,
    MyPrimitiveDataTypes::Float,
    arrow::datatypes::Float32Type,
    "A 32bit float type."
);
make_arrow_type!(
    MyArrowDoubleType,
    MyPrimitiveDataTypes::Double,
    arrow::datatypes::Float64Type,
    "A 64bit Double type."
);

// The entry point for primtive array builders
pub struct MyPrimitiveArrowArrayBuilder<T: MyArrowDataTypePrimitives> {
    builder: arrow::array::PrimitiveBuilder<T::MyNative>,
}

// implement the array primitive builder's trait
impl<T: MyArrowDataTypePrimitives> MyArrayBuilderInterface for MyPrimitiveArrowArrayBuilder<T> {
    const BUILDER_TYPE: Format = Format::ARROW;
    const DATA_TYPE: MyPrimitiveDataTypes = T::DATA_TYPE;

    // this needs to be the native type from Arrow
    type ValueType = <<T as MyArrowDataTypePrimitives>::MyNative as ArrowPrimitiveType>::Native;
    type ReturnTypeAtBuild = arrow::array::PrimitiveArray<T::MyNative>;

    fn append(&mut self, value: Self::ValueType) {
        self.builder.append_value(value);
    }

    fn finish(&mut self) -> MyArray<Self::ReturnTypeAtBuild> {
        MyArray::from_native(self.builder.finish())
    }

    fn with_capacity(initial_capacity: usize) -> Self {
        let builder =
            arrow::array::PrimitiveBuilder::<T::MyNative>::with_capacity(initial_capacity);
        Self { builder }
    }
}
