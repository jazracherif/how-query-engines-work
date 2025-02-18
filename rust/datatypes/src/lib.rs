mod column_vector;
mod data_types;
mod field_vector_arrow;
mod field_vector_base;
mod field_vector_factory;
mod literal_value_vector;
mod record_batch;

#[cfg(test)]
mod tests {
    // use crate::field_vector_factory::ArrowBuildersTrait;
    use arrow::datatypes::Int32Type;
    use field_vector_arrow::{MyArrowFloatType, MyArrowInt32Type};
    use field_vector_base::MyArrayBuilderInterface;

    use super::*;

    #[test]
    fn test_bare_arrow() {
        // create an int vector
        let mut int_array_builder = arrow::array::PrimitiveBuilder::<Int32Type>::with_capacity(3);
        int_array_builder.append_option(Some(1));
        int_array_builder.append_option(Some(2));
        int_array_builder.append_option(Some(3));

        let int_array = int_array_builder.finish();

        let collected: Vec<_> = int_array.iter().collect();
        assert_eq!(collected, vec![Some(1), Some(2), Some(3)]);
    }

    // #[test]
    // fn test_factory_arrow_primitives_int32() {
    //     let mut bldr =
    //         field_vector_factory::ArrayBuilderFactory::build_arrow_primitive::<Int32Type>(3);
    //     bldr.append(1);
    //     bldr.append(2);
    //     bldr.append(3);

    //     let int_array = bldr.build();

    //     let collected: Vec<_> = int_array.get_arrow().iter().collect();
    //     assert_eq!(collected, vec![Some(1), Some(2), Some(3)]);
    // }

    // #[test]
    // fn test_factory_arrow_primitives_bool() {
    //     let mut bldr = field_vector_factory::ArrayBuilderFactory::build_arrow_boolean(3);
    //     bldr.append(true);
    //     bldr.append(false);
    //     bldr.append(false);

    //     let int_array = bldr.build();

    //     let collected: Vec<_> = int_array.get_arrow().iter().collect();
    //     assert_eq!(collected, vec![Some(true), Some(false), Some(false)]);
    // }

    #[test]
    fn test_factory_generic_primitives_int32() {
        let mut bldr =
            field_vector_factory::ArrayBuilderFactory::primitive_array::<MyArrowInt32Type>(3);
        bldr.append(1);
        bldr.append(2);
        bldr.append(3);

        let int_array = bldr.finish();

        let collected: Vec<_> = int_array.get_native_array().iter().collect();
        assert_eq!(collected, vec![Some(1), Some(2), Some(3)]);
    }

    #[test]
    fn test_factory_generic_primitives_float() {
        let mut bldr =
            field_vector_factory::ArrayBuilderFactory::primitive_array::<MyArrowFloatType>(3);
        bldr.append(1.0);
        bldr.append(2.0);
        bldr.append(3.0);

        let int_array = bldr.finish();

        let collected: Vec<_> = int_array.get_native_array().iter().collect();
        assert_eq!(collected, vec![Some(1.0), Some(2.0), Some(3.0)]);
    }
}
