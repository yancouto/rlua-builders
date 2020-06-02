use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed,
    Ident, Index,
};

fn builder_for_unnamed(name: TokenStream2, fields: &FieldsUnnamed) -> TokenStream2 {
    let i = (0..fields.unnamed.len()).map(Index::from);
    let types = fields.unnamed.iter().map(|f| &f.ty);
    quote! {
        ctx.create_function(|_, args: (#(#types,)*)| {
            Ok(#name (#(args.#i,)*))
        })
    }
}

fn builder_for_named(name: TokenStream2, fields: &FieldsNamed) -> TokenStream2 {
    let names = fields.named.iter().map(|x| &x.ident);
    let types = fields.named.iter().map(|x| &x.ty);

    quote! {
        ctx.create_function(|_, data: rlua::Table<'s>| {
            Ok(#name {
                #( #names: data.get::<_, #types>(stringify!(#names))?, )*
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

#[proc_macro_derive(LuaBuilder)]
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

#[proc_macro_derive(UserData)]
pub fn derive_user_data(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl UserData for #name {}
    };

    TokenStream::from(expanded)
}
