use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type};

mod check_attr;
mod check_fields;

#[proc_macro_derive(HexDump)]
pub fn hex_dump_derive_macro(input: TokenStream) -> TokenStream{
    
    let ast: DeriveInput = syn::parse(input).unwrap();
    
    match impl_hex_dump_macro(&ast){
        Ok(t) => t.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn impl_hex_dump_macro(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream>{
    
    check_attr::check_C_packed(input)?;


    let fields = match &input.data{
        Data::Struct(e) => match &e.fields{
            Fields::Named(f) => &f.named,
            _ => return Err(syn::Error::new_spanned(&input.ident, "HexDump can only derive field known structs")),
        }

        _ => return Err(syn::Error::new_spanned(&input.ident, "HexDump can only be derived foer structs!"))
    };


    let entries = fields
        .iter()
        .map(|field| -> syn::Result<proc_macro2::TokenStream> {
            let ident = field.ident.as_ref().unwrap();
            let ty = &field.ty;

            let primitive_type = match ty {
                Type::Path(p) => p
                    .path
                    .segments
                    .last()
                    .map(|s| s.ident.to_string())
                    .ok_or_else(|| syn::Error::new_spanned(ty, "couldn't find the type"))?,
                _ => return Err(syn::Error::new_spanned(ty, "unsupported field type")),
            };

            let struct_type = match primitive_type.as_str() {
                "u8"  => quote! { ::hex_dump_core::struct_type::StructType::U8 },
                "u16" => quote! { ::hex_dump_core::struct_type::StructType::U16 },
                "u32" => quote! { ::hex_dump_core::struct_type::StructType::U32 },
                "u64" => quote! { ::hex_dump_core::struct_type::StructType::U64 },
                "i8"  => quote! { ::hex_dump_core::struct_type::StructType::I8 },
                "i16" => quote! { ::hex_dump_core::struct_type::StructType::I16 },
                "i32" => quote! { ::hex_dump_core::struct_type::StructType::I32 },
                "i64" => quote! { ::hex_dump_core::struct_type::StructType::I64 },
                "f32" => quote! { ::hex_dump_core::struct_type::StructType::F32 },
                "f64" => quote! { ::hex_dump_core::struct_type::StructType::F64 },
                _ => return Err(syn::Error::new_spanned(ty, "unsupported field type")),
            };

            Ok(quote! {
                ::hex_dump_core::hex_dump_field_info::FieldInfo {
                    name: stringify!(#ident),
                    type_field: #struct_type,
                    offset: ::core::mem::offset_of!(Self, #ident),
                    size: ::core::mem::size_of::<#ty>(),
                }
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;

    let name = &input.ident;
    Ok(quote! {
        unsafe impl ::bytemuck::Zeroable for #name{}
        unsafe impl ::bytemuck::Pod for #name{}

        impl ::hex_dump_core::HexDump for #name{
            const FIELDS: &'static [::hex_dump_core::hex_dump_field_info::FieldInfo] = &[
                #(#entries),*
            ];
        }
    })
}
