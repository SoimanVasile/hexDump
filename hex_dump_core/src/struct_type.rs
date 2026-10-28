use std::num::NonZeroU64;

use bytemuck::{Pod, Zeroable};
use crate::value::Value;

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
}



impl StructType{
    pub fn get_size(&self) -> usize{
        match self{
            Self::U8 => 1,
            Self::U16 => 2,
            Self::U32 => 4,
            Self::U64 => 8,
            Self::I8 =>  1,
            Self::I16 => 2,
            Self::I32 => 4,
            Self::I64 => 8,
            Self::F32 => 4,
            Self::F64 => 8,
        }
    }

    pub fn representation<T: Pod + Zeroable>(&self, bytes: &[u8]) -> Result<T, ()>{
        if bytes.len() != size_of::<T>(){
            return Err(());
        }

        Ok(*bytemuck::from_bytes(bytes))
    }
    pub fn decode(&self, bytes: &[u8]) -> Value  {
    macro_rules! arm {
        ($variant:ident, $t:ty) => {{
            let v = <$t>::from_ne_bytes(bytes.try_into().unwrap()); 
            Value::$variant(v)                    
        }};
    }
    match self {
        Self::U8  => arm!(U8, u8),   Self::I8  => arm!(I8, i8),
        Self::U16 => arm!(U16, u16), Self::I16 => arm!(I16, i16),
        Self::U32 => arm!(U32, u32), Self::I32 => arm!(I32, i32),
        Self::U64 => arm!(U64, u64), Self::I64 => arm!(I64, i64),
        Self::F32 => arm!(F32, f32), Self::F64 => arm!(F64, f64),
    }
}
}
