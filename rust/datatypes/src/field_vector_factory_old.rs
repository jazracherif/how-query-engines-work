// TODO: REMOVE

use crate::data_types::MyArrayBuilderInterface;
use crate::field_vector_arrow::{MyArrowDataTypePrimitives, MyPrimitiveArrowArrayBuilder};
use arrow::{
    datatypes::{self, *},
    ipc::{Bool, BoolBuilder},
};

/// create a an initial Array vector, an implementation of the ColumnVector which takes care of the
/// casting work

pub trait ArrowBuildersTrait {
    type ValueType;
    type ReturnType;

    fn append(&mut self, value: Self::ValueType);
    fn build(&mut self) -> ArrowArray<Self::ReturnType>;
}

pub struct ArrowArray<T> {
    data: T,
}

impl<T> ArrowArray<T> {
    pub fn get_arrow(&self) -> &T {
        &self.data
    }
}

pub struct BooleanBuilder {
    builder: arrow::array::BooleanBuilder,
}

impl ArrowBuildersTrait for BooleanBuilder {
    type ValueType = bool;
    type ReturnType = arrow::array::BooleanArray;

    fn append(&mut self, value: Self::ValueType) {
        self.builder.append_value(value);
    }

    fn build(&mut self) -> ArrowArray<Self::ReturnType> {
        ArrowArray {
            data: self.builder.finish(),
        }
    }
}

pub struct PrimitiveArrayBuilder<T: ArrowPrimitiveType> {
    builder: arrow::array::PrimitiveBuilder<T>,
}

impl<T: ArrowPrimitiveType> ArrowBuildersTrait for PrimitiveArrayBuilder<T> {
    type ValueType = T::Native;
    type ReturnType = arrow::array::PrimitiveArray<T>;

    fn append(&mut self, value: Self::ValueType) {
        self.builder.append_value(value);
    }

    fn build(&mut self) -> ArrowArray<Self::ReturnType> {
        ArrowArray {
            data: self.builder.finish(),
        }
    }
}

pub struct ArrayBuilderFactory {}
impl ArrayBuilderFactory {
    pub fn build_arrow_primitive<T: ArrowPrimitiveType>(
        initial_capacity: usize,
    ) -> PrimitiveArrayBuilder<T> {
        let builder: arrow::array::PrimitiveBuilder<T> =
            arrow::array::PrimitiveBuilder::<T>::with_capacity(initial_capacity);
        return PrimitiveArrayBuilder { builder };
    }

    pub fn build_arrow_boolean(initial_capacity: usize) -> BooleanBuilder {
        let builder = arrow::array::BooleanBuilder::with_capacity(initial_capacity);
        return BooleanBuilder { builder };
    }

    /// Arrow Methods
    pub fn build_primitive<T: MyArrowDataTypePrimitives>(
        initial_capacity: usize,
    ) -> MyPrimitiveArrowArrayBuilder<T> {
        return MyPrimitiveArrowArrayBuilder::<T>::with_capacity(initial_capacity);
    }
}

// //
// struct ArrowFieldVector {
//     field: Arrow,
// }

// //  the
// impl ColumnVector for ArrowFieldVector {
//     fn get_type(&self) -> &ArrowTypes::ArrowTypes;
//     fn get_value(&self, i: i32) -> Result<&Box<dyn Any>, String>;
//     fn size(&self) -> i32;
// }
