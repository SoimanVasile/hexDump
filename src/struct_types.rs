use std::num::NonZeroU64;

pub enum StructType{
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    CharCounter(Option<NonZeroU64>),
}



impl StructType{
    pub fn get_size(&self) -> i64{
        match self{
            Self::U8 => 8,
            Self::U16 => 16,
            Self::U32 => 32,
            Self::U64 => 64,
            Self::I8 =>  8,
            Self::I16 => 16,
            Self::I32 => 32,
            Self::I64 => 64,
            Self::F32 => 32,
            Self::F64 => 64,
            Self::CharCounter(length) => { if let Some(len) = length{
                    len.get() as i64
            } else {
                -1
            }}
            // _ => i64::MAX,
        }
    }
}
