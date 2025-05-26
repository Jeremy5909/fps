use std::fmt::Debug;

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;
use syn::ExprLit;
use syn::Fields;
use syn::parse_macro_input;

#[proc_macro_derive(VertexAttribPointers, attributes(location))]
pub fn derive_vertex_attrib_pointers(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);
    generate_impl(&ast).into()
}

fn generate_impl(input: &DeriveInput) -> proc_macro2::TokenStream {
    let ident = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields_vertex_attrib_pointer = generate_vertex_attrib_pointer(&input.data);

    quote! {
        impl #impl_generics vertex_attrib::VertexAttribPointers for #ident #ty_generics #where_clause {
            #[allow(unused_variables)]
            fn vertex_attrib_pointers() {
                let stride = ::std::mem::size_of::<Self>();
                let mut offset = 0;

                #(#fields_vertex_attrib_pointer)*
            }
        }
    }
}

fn generate_vertex_attrib_pointer(data: &syn::Data) -> Vec<proc_macro2::TokenStream> {
    match data {
        &syn::Data::Struct(ref data_struct) => match &data_struct.fields {
            Fields::Named(named) => named
                .named
                .iter()
                .map(generate_struct_field_vertex_attrib_pointer_call)
                .collect(),
            _ => Err(Error::NotNamedStruct).unwrap(),
        },
        _ => Err(Error::NotNamedStruct).unwrap(),
    }
}

fn generate_struct_field_vertex_attrib_pointer_call(
    field: &syn::Field,
) -> proc_macro2::TokenStream {
    let field_name = field
        .ident
        .as_ref()
        .map(|i| i.to_string())
        .unwrap_or(String::from("<unnamed>"));
    let location_attr = field
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("location"))
        .ok_or(Error::AttributeMissing(field_name.clone()))
        .unwrap();
    let location_value: usize = match location_attr
        .meta
        .require_name_value()
        .map_err(|_| Error::AttributeFormatIncorrect(field_name.clone()))
        .unwrap()
        .value
    {
        syn::Expr::Lit(ExprLit {
            lit: syn::Lit::Int(ref s),
            ..
        }) => s
            .base10_parse()
            .map_err(|_| Error::AttributeExpectedIntLiteral(field_name.clone()))
            .unwrap(),
        _ => Err(Error::AttributeExpectedIntLiteral(field_name.clone())).unwrap(),
    };
    let ty = &field.ty;
    quote! {
        let location = #location_value;
        unsafe {
            Vertex::vertex_attrib_pointer(stride, location, offset);
        }
        offset += ::std::mem::size_of::<#ty>();
    }
}

enum Error {
    AttributeMissing(String),
    AttributeFormatIncorrect(String),
    AttributeExpectedIntLiteral(String),
    NotNamedStruct,
}
impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            &Self::AttributeMissing(field_name) => {
                write!(f, "Field '{field_name}' missing attribute #[location = ..]")
            }
            &Self::AttributeFormatIncorrect(field_name) => write!(
                f,
                "Attribute in field '{field_name}' doesn't have form #[location = ..]"
            ),
            &Self::AttributeExpectedIntLiteral(field_name) => {
                write!(f, "Field '{field_name}' pos must be int")
            }
            &Self::NotNamedStruct => write!(
                f,
                "VertexAttribPointers can only be implemented for named structs"
            ),
        }
    }
}
