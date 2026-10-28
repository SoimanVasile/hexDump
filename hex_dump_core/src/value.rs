use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    U8(u8), U16(u16), U32(u32), U64(u64),
    I8(i8), I16(i16), I32(i32), I64(i64),
    F32(f32), F64(f64),
}


impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::U8(v)  => fmt::Display::fmt(v, f),
            Value::U16(v) => fmt::Display::fmt(v, f),
            Value::U32(v) => fmt::Display::fmt(v, f),
            Value::U64(v) => fmt::Display::fmt(v, f),
            Value::I8(v)  => fmt::Display::fmt(v, f),
            Value::I16(v) => fmt::Display::fmt(v, f),
            Value::I32(v) => fmt::Display::fmt(v, f),
            Value::I64(v) => fmt::Display::fmt(v, f),
            Value::F32(v) => fmt::Display::fmt(v, f),
            Value::F64(v) => fmt::Display::fmt(v, f),
        }
    }
}
