use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Ident, Index};

fn builder_for_unnamed(name: TokenStream2, fields: &FieldsUnnamed) -> TokenStream2 {
    let i = (0..fields.unnamed.len()).map(Index::from);
    quote! {
        ctx.create_function(|_, args: #fields| {
            Ok(#name (#(args.#i,)*))
        })
    }
}

fn builder_for_named(name: TokenStream2, fields: &FieldsNamed) -> TokenStream2 {
    let names = fields.named.iter().map(|x| x.ident.clone());
    let types = fields.named.iter().map(|x| x.ty.clone());

    quote! {
        ctx.create_function(|_, data: rlua::Table<'s>| {
            Ok(#name {
                #(
                    #names: data.get::<_, #types>(stringify!(#names))?,
                )*
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
        impl<'s> LuaStructBuilder<'s, rlua::Function<'s>> for #name {
            fn builder(ctx: rlua::Context<'s>) -> rlua::Result<rlua::Function<'s>> {
                #builder
            }
        }
    }
}

fn self_struct_builder(name: Ident, builder: TokenStream2) -> TokenStream2 {
    quote! {
        impl<'s> LuaStructBuilder<'s, Self> for #name {
            fn builder(ctx: rlua::Context<'s>) -> rlua::Result<Self> {
                #builder
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

    let code = builder_for_fields(quote! {Self}, &ds.fields);

    let code = match ds.fields {
        Fields::Unit => self_struct_builder(name, code),
        Fields::Unnamed(..) | Fields::Named(..) => function_struct_builder(name, code),
    };

    TokenStream::from(code)
}

#[proc_macro_derive(LuaEnumBuilder)]
pub fn derive_enum_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let de = match input.data {
        Data::Enum(de) => de,
        _ => panic!("Must annotate enum"),
    };

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

    let code = quote! {
        impl<'s> LuaEnumBuilder<'s> for #name {
            fn builder(ctx: rlua::Context<'s>) -> rlua::Result<rlua::Table<'s>> {
                let t = ctx.create_table()?;
                #( t.set(stringify!(#names), #builders?)?; )*
                Ok(t)
            }
        }
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
