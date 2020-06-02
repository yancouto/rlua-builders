use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Ident, FieldsUnnamed, FieldsNamed, Index};
use proc_macro2::{TokenStream as TokenStream2};
use quote::quote;


fn from_unit(name: Ident) -> TokenStream2 {
    quote! {
        impl<'s> LuaStructBuilder<'s, Self> for #name {
            fn builder(ctx: rlua::Context<'s>) -> rlua::Result<Self> {
                Ok(Self)
            }
        }
    }
}

fn from_unnamed(name: Ident, fields: FieldsUnnamed) -> TokenStream2 {
    let i = (0..fields.unnamed.len()).map(Index::from);
    quote! {
        impl<'s> LuaStructBuilder<'s, rlua::Function<'s>> for #name {
            fn builder(ctx: rlua::Context<'s>) -> rlua::Result<rlua::Function<'s>> {
                ctx.create_function(|_, args: #fields| {
                    Ok(Self(#(args.#i,)*))
                })
            }
        }
    }
}

fn from_named(name: Ident, fields : FieldsNamed) -> TokenStream2 {
    let names = fields.named.iter().map(|x| x.ident.clone());
    let types = fields.named.iter().map(|x| x.ty.clone());

    quote! {
        impl<'s> LuaStructBuilder<'s, rlua::Function<'s>> for #name {
            fn builder(ctx: rlua::Context<'s>) -> rlua::Result<rlua::Function<'s>> {
                ctx.create_function(|_, data: rlua::Table<'s>| {
                    Ok(Self {
                        #(
                            #names: data.get::<_, #types>(stringify!(#names))?,
                        )*
                    })
                })
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
        Fields::Unnamed(unnamed) => from_unnamed(name, unnamed),
        Fields::Named(named) => from_named(name, named),
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