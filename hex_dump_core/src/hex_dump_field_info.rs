use crate::struct_type::StructType;

pub struct FieldInfo{
    pub(crate) name: String,
    pub(crate) type_field: StructType,
    pub(crate) offset: usize,
}
