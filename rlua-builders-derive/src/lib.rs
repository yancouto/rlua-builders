use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Ident};
use proc_macro2::{TokenStream as TokenStream2};
use quote::quote;


fn from_unit(name: Ident) -> TokenStream2 {
    quote! {
        impl<'s> LuaStructBuilder<'s, Self> for #name {
            fn builder(ctx: Context<'s>) -> Self {
                Self
            }
        }
    }
}

#[proc_macro_derive(LuaStructBuilder)]
pub fn derive_struct_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let ds = match input.data {
        Data::Struct(ds) => ds,
        _ => panic!("Must annotate struct"),
    };

    let code = match ds.fields {
        Fields::Unit => from_unit(name),
        _ => panic!("Must be unit!"),
    };

    TokenStream::from(code)
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