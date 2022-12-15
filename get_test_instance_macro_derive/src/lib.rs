use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, Data, DeriveInput, Field, Fields};

// Implementation of the trait [GetTestInstance](`starknet_api::test_utils::GetTestInstance`)
// for starknet_api structs and enums. Should create valid, non-empty, and non-trivial instances
// for testing.
// To derive this implementation add #[cfg_attr(feature = "testing", derive(GetTestInstance))].
#[proc_macro_derive(GetTestInstance)]
pub fn get_test_instance_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse(input).unwrap();
    let name = &ast.ident;
    match ast.data {
        Data::Struct(data) => {
            let field_tokens =
                data.fields.iter().map(|f| impl_get_test_instance_for_field(f.clone()));
            let self_tokens = if let Fields::Unnamed(_) = data.fields {
                quote! {
                    Self(
                        #(#field_tokens, )*
                    )
                }
            } else {
                quote! {
                    Self {
                        #(#field_tokens, )*
                    }
                }
            };
            let gen = quote! {
                impl GetTestInstance for #name {
                    fn get_test_instance() -> Self {
                        #self_tokens                    }
                }
            };
            TokenStream::from(gen)
        }
        _ => panic!("Not supported yet."),
    }
}

fn impl_get_test_instance_for_field(field: Field) -> proc_macro2::TokenStream {
    let mut tokens = proc_macro2::TokenStream::new();
    if let Some(ident) = field.ident {
        tokens.extend(quote!(#ident: ));
    }

    let ty = field.ty;
    if let syn::Type::Path(tp) = &ty {
        if tp.path.segments.len() == 1 {
            let type_name = tp.path.segments[0].ident.to_string();
            // Primitive types.
            if type_name.to_lowercase() == type_name {
                tokens.extend(quote!(#ty::default()));
                return tokens;
            }
            // StarkHash and StarkFelt.
            if type_name == "StarkHash" || type_name == "StarkFelt" {
                tokens.extend(quote!(crate::shash!("0x1")));
                return tokens;
            }
            // PatriciaKey.
            if type_name == "PatriciaKey" {
                tokens.extend(quote!(crate::patky!("0x1")));
                return tokens;
            }
        }
    }

    // Other.
    tokens.extend(quote!(#ty::get_test_instance()));
    tokens
}
