use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, DeriveInput};

#[proc_macro_derive(GetTestInstance)]
pub fn get_test_instance(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse(input).unwrap();
    let name = &ast.ident;
    match ast.data {
        syn::Data::Struct(data) => match data.fields {
            syn::Fields::Unnamed(fields) => {
                let tys = fields.unnamed.iter().map(|f| f.ty.clone());
                let gen = quote! {
                    impl GetTestInstance for #name {
                        fn get_test_instance() -> Self {
                            Self(
                                #(#tys::get_test_instance(), )*
                            )
                        }
                    }
                };
                TokenStream::from(gen)
            }
            syn::Fields::Named(fields) => {
                let idents = fields.named.iter().map(|f| f.ident.clone());
                let tys = fields.named.iter().map(|f| f.ty.clone());
                let gen = quote! {
                    impl GetTestInstance for #name {
                        fn get_test_instance() -> Self {
                            Self {
                                #(#idents: #tys::get_test_instance(), )*
                            }
                        }
                    }
                };
                TokenStream::from(gen)
            }
            _ => panic!("Not supported yet."),
        },
        _ => panic!("Not supported yet."),
    }
}
