//! The rlua-builders-derive crate provides derivers for the [`rlua-builders`] crate.
//!
//! This crate provides a deriver for [`LuaBuilder`] from [`rlua-builders`], as well
//! as for [`UserData`] from [`rlua`].
//! This is not usually imported directly. See [`rlua-builders`] for more
//! documentation.
//!
//! [`UserData`]: https://docs.rs/rlua/*/rlua/trait.UserData.html
//! [`LuaBuilder`]: https://docs.rs/rlua-builders/*/rlua_builders/trait.LuaBuilder.html
//! [`rlua-builders`]: https://crates.io/crates/rlua-builders
//! [`rlua`]: https://crates.io/crates/rlua

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Field, Fields, FieldsNamed,
    FieldsUnnamed, Ident, Index, Meta, MetaNameValue,
};

const DEFAULT: &'static str = "default";

fn create_type_and_unwrap<'s>(
    fields: impl Iterator<Item = &'s Field>,
) -> (Vec<TokenStream2>, Vec<TokenStream2>) {
    fields
        .map(|f| {
            let ty = &f.ty;
            const SYNTAX_ERR: &'static str = "Invalid syntax for default";
            let def = f.attrs.iter().find(|a| a.path.is_ident(DEFAULT)).map(|a| {
                match a.parse_meta().expect(SYNTAX_ERR) {
                    Meta::NameValue(MetaNameValue { lit, .. }) => quote!( .unwrap_or(#lit) ),
                    _ => panic!(SYNTAX_ERR),
                }
            });
            match def {
                Some(ts) => (quote! { ::std::option::Option<#ty> }, ts),
                None => (quote! (#ty), quote!()),
            }
        })
        .unzip()
}

fn builder_for_unnamed(name: TokenStream2, fields: &FieldsUnnamed) -> TokenStream2 {
    let i = (0..fields.unnamed.len()).map(Index::from);
    let (types, unwraps): (Vec<_>, Vec<_>) = create_type_and_unwrap(fields.unnamed.iter());
    quote! {
        ctx.create_function(|_, args: (#(#types,)*)| {
            Ok(#name (#(args.#i #unwraps ,)*))
        })
    }
}

fn builder_for_named(name: TokenStream2, fields: &FieldsNamed) -> TokenStream2 {
    let names = fields.named.iter().map(|x| &x.ident);
    let (types, unwraps): (Vec<_>, Vec<_>) = create_type_and_unwrap(fields.named.iter());

    quote! {
        ctx.create_function(|_, data: rlua::Table<'s>| {
            Ok(#name {
                #( #names: data.get::<_, #types>(stringify!(#names))? #unwraps , )*
            })
        })
    }
}

fn builder_for_fields(name: TokenStream2, fields: &Fields) -> TokenStream2 {
    match fields {
        Fields::Unit => quote! { Ok(#name) },
        Fields::Unnamed(unnamed) => builder_for_unnamed(name, unnamed),
        Fields::Named(named) => builder_for_named(name, named),
    }
}

fn function_struct_builder(name: Ident, builder: TokenStream2) -> TokenStream2 {
    quote! {
        impl<'s> LuaBuilder<'s, rlua::Function<'s>> for #name {
            fn builder(ctx: rlua::Context<'s>) -> rlua::Result<rlua::Function<'s>> {
                #builder
            }
        }
    }
}

fn self_struct_builder(name: Ident, builder: TokenStream2) -> TokenStream2 {
    quote! {
        impl<'s> LuaBuilder<'s, Self> for #name {
            fn builder(ctx: rlua::Context<'s>) -> rlua::Result<Self> {
                #builder
            }
        }
    }
}

fn struct_builder(name: Ident, ds: DataStruct) -> TokenStream2 {
    let code = builder_for_fields(quote! {Self}, &ds.fields);

    match ds.fields {
        Fields::Unit => self_struct_builder(name, code),
        Fields::Unnamed(..) | Fields::Named(..) => function_struct_builder(name, code),
    }
}

fn enum_builder(name: Ident, de: DataEnum) -> TokenStream2 {
    let (names, builders): (Vec<_>, Vec<_>) = de
        .variants
        .iter()
        .map(|v| {
            let var_name = &v.ident;
            (
                var_name,
                builder_for_fields(quote! {#name::#var_name}, &v.fields),
            )
        })
        .unzip();

    quote! {
        impl<'s> LuaBuilder<'s, rlua::Table<'s>> for #name {
            fn builder(ctx: rlua::Context<'s>) -> rlua::Result<rlua::Table<'s>> {
                let t = ctx.create_table()?;
                #( t.set(stringify!(#names), #builders?)?; )*
                Ok(t)
            }
        }
    }
}

/// Automatically derive the [`LuaBuilder`] trait for structs and enums
///
/// See the [`rlua-builders`] documentation for specifics of how this works.
///
/// [`LuaBuilder`]: https://docs.rs/rlua-builders/*/rlua_builders/trait.LuaBuilder.html
/// [`rlua-builders`]: https://crates.io/crates/rlua-builders
#[proc_macro_derive(LuaBuilder, attributes(default))]
pub fn derive_struct_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let code = match input.data {
        Data::Struct(ds) => struct_builder(name, ds),
        Data::Enum(de) => enum_builder(name, de),
        _ => panic!("Must annotate struct or enum"),
    };

    TokenStream::from(code)
}

/// Automatically derive [`UserData`] trait
///
/// This derive macro derives an **empty** [`UserData`] for the struct or enum.
/// That means it won't have any custom methods. This is separate from the
/// [`LuaBuilder`] deriver in case you want to derive [`UserData`] with a custom
/// implementation.
///
/// [`UserData`]: https://docs.rs/rlua/*/rlua/trait.UserData.html
/// [`LuaBuilder`]: https://docs.rs/rlua-builders/*/rlua_builders/trait.LuaBuilder.html
#[proc_macro_derive(UserData)]
pub fn derive_user_data(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl UserData for #name {}
    };

    TokenStream::from(expanded)
}
