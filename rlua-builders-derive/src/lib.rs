use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Data, Fields};
use quote::quote;

#[proc_macro_derive(LuaStructBuilder)]
pub fn derive_struct_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let ds = match input.data {
        Data::Struct(ds) => ds,
        _ => panic!("Must annotate struct"),
    };

    match ds.fields {
        Fields::Unit => (),
        _ => panic!("Must be unit!"),
    };

    let expanded = quote! {
        impl<'s> LuaStructBuilder<'s, Self> for #name {
            fn builder(ctx: Context<'s>) -> Self {
                Self
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(UserData)]
pub fn derive_user_data(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl UserData for #name {}
    };

    TokenStream::from(expanded)
}