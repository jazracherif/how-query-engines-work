// use crate::ColumnVector::ColumnVector;
// use crate::DataTypes::DataTypes;
// use std::any::Any;

// /// represents a vector with the same value across the whole vector
// struct LiteralValueVector {
//     arrow_type: DataTypes,
//     value: Box<dyn Any>,
//     size: i32,
// }

// impl ColumnVector for LiteralValueVector{
//     fn get_type(&self) -> &DataTypes {
//         &self.arrow_type
//     }

//     fn get_value(&self, i: i32) -> Result<&Box<dyn Any>, String> {
//         if i < 0 || i > self.size {
//             return Err("IndexOutOfBound".to_string());
//         }
//         Ok(&self.value)
//     }

//     fn size(&self) -> i32 {
//         self.size
//     }
// }
