use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;
use syn::parse_macro_input;

#[proc_macro_derive(VertexAttribPointers, attributes())]
pub fn derive_vertex_attrib_pointers(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);
    generate_impl(&ast).into()
}

fn generate_impl(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    quote! {
        impl #name {
            pub fn vertex_attrib_pointers() {
                let stride = mem::size_of::<Self>();

                unsafe {
                    Self::vertex_attrib_pointer(stride, 0, 0);
                    Self::vertex_attrib_pointer(stride, 1, 3 * std::mem::size_of::<f32>());
                }
            }
        }
    }
}
