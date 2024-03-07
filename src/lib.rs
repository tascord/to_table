use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Ident};

fn preamble(input: DeriveInput) -> (DeriveInput, Ident, DataStruct) {
    let name = input.clone().ident;
    let data = match input.clone().data {
        Data::Struct(data) => data,
        _ => panic!("ToTable can only be implemented for structs"),
    };

    (input, name, data)
}

#[proc_macro_derive(ToTable, attributes(skip, unwrap))]
pub fn tt_derive(input: TokenStream) -> TokenStream {
    let (_, name, data) = preamble(parse_macro_input!(input as DeriveInput));
    let fields = data
        .fields
        .iter()
        .filter(|f| !f.attrs.iter().any(|a| a.path().is_ident("skip")))
        .collect::<Vec<_>>();

    let should_unwrap = data
        .fields
        .iter()
        .filter(|f| f.attrs.iter().any(|a| a.path().is_ident("unwrap")))
        .map(|f| f.ident.clone())
        .collect::<Vec<_>>();

    let defaults = fields
        .iter()
        .map(|f| f.ident.clone())
        .filter(|f| !should_unwrap.contains(f));

    let implimentation = quote! {
        impl TableData for #name {
            fn sort(&self, other: &Self, field: String) -> std::cmp::Ordering {
                match field.as_str() {
                    #(
                        stringify!(#defaults) => self.#defaults.cmp(&other.#defaults),
                    )*
                    #(
                        stringify!(#should_unwrap) => self.#should_unwrap.partial_cmp(&other.#should_unwrap).unwrap(),
                    )*
                    _ => std::cmp::Ordering::Equal,
                }
            }
        }
    };

    TokenStream::from(implimentation)
}
