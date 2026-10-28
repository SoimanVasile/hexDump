use crate::hex_dump_field_info::FieldInfo;
use std::fmt::Write;

pub trait HexDump{
    const FIELDS: &'static [FieldInfo];
    const LINE: &'static str = "-----------------------------------------------------------";

    fn hex_dump(bytes: &[u8], row: usize) -> Result<String, ()>{
        let mut out = String::new();
        writeln!(out, "{}", Self::LINE);
        
        for (index, f) in Self::FIELDS.iter().enumerate(){
            write!(out, "{:>6} ", row + index );
            let slice = &bytes[f.offset .. f.offset + f.type_field.get_size() as usize];
            let nr = f.type_field.decode(slice);
            for b in bytes{
                write!(out, "{b:02x} ");
            }

            write!(out, " | {}: {}", f.name, nr);
            write!(out, "\n");
        }

        writeln!(out, "--------------------------------------------------------");
        Ok(out)
    }
}
