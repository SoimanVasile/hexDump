use crate::struct_types::StructType;

pub struct FieldInfo{
    name: String,
    type_field: StructType,
    offset: usize,
}


pub trait HexDump{
    const FIELDS: &'static [FieldInfo];

    pub fn hex_dump(bytes: &[u8]) -> Result<String>{
        let mut out = String::new();

        for f in Self::FIELDS{
            let slice = bytes[f.offset .. f.offset + f.type_field.get_size() as usize];
            let nr = f.type_field.get_representation(slice);
        }

        Ok(out)
    }
}
