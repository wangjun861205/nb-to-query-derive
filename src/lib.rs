use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, Type};

#[proc_macro_derive(ToQueryDerive)]
pub fn to_query_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    if let Data::Struct(stct) = input.data {
        let mut pairs = Vec::new();
        for field in stct.fields {
            if let Some(ident) = field.ident {
                match field.ty {
                    Type::Path(_) | Type::Reference(_) => pairs.push(quote! {
                        pairs.push(self.#ident.to_query(stringify!(#ident)));
                    }),
                    _ => panic!("unsupported field type: {}", ident),
                }
            }
        }
        let name = input.ident;
        return quote! {
            impl ToQuery for #name {
                fn to_query(&self, _: &str) -> Option<String> {
                    let mut pairs = Vec::new();
                    #(#pairs)*
                    if pairs.is_empty() {
                        return None;
                    }
                    let pairs: Vec<String> = pairs.into_iter().flat_map(|p| p).collect();
                    Some(pairs.join("&"))
                }
            }
        }
        .into();
    }
    panic!("only support struct")
}
