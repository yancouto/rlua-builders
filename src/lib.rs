//! The rlua-builders crate provides helpers for Rust/Lua interop using
//! [`rlua`]. That means creating Rust struct/enums from Lua with almost the
//! same syntax as in Rust!
//!
//! This crate itself only provides a trait definition, but [`rlua-builders-derive`]
//! provides derive macros to automatically implement the defined trait for
//! any struct or enum. It just needs to have [`UserData`] implemented for it. After
//! that, simply call `StructOrEnum::builder(ctx)` to create a builder for that struct
//! or enum.
//!
//! ## What do these builders look like?
//! - For [unit structs]: A builder is simply the unit value itself, which can be converted to
//! Lua since the struct implements [`UserData`].
//! - For [tuple structs]: A builder is a lua function that receives the tuple arguments in order
//! and returns the userdata for the struct.
//! - For [normal structs]: A builder is a lua function that receives a single table with
//! the named arguments to the struct.
//! - For [enums]: A builder is a table where each field is a builder for each of the enum's
//! variants, and each of them works the same way as the struct builders defined above.
//!
//! ## Examples:
//! This shows how to derive and use [`LuaBuilder`] on a simple struct. See [`rlua`] for
//! more documentation on how to interop with Lua.
//! ```
//! use rlua::UserData;
//! use rlua_builders::LuaBuilder;
//! use rlua_builders_derive::{LuaBuilder, UserData};
//!
//! #[derive(LuaBuilder, UserData, Clone, PartialEq, Debug)]
//! struct Person {
//!     name: String,
//!     age: u8,
//! }
//!
//! let p = rlua::Lua::new().context(|ctx| {
//!     ctx.globals().set("Person", Person::builder(ctx)?)?;
//!     ctx.load(r#"Person { name = "Yan", age = 24 }"#).eval::<Person>()
//! }).unwrap();
//!
//! assert_eq!(p, Person {name: "Yan".to_owned(), age: 24})
//! ```
//!
//! Enums work in a similar way, except their constructor is a table where each function
//! is equivalent to a struct builder.
//!
//! ```
//! # use rlua::UserData;
//! # use rlua_builders::LuaBuilder;
//! # use rlua_builders_derive::*;
//! #[derive(LuaBuilder, UserData, Clone)]
//! enum Valuables {
//!     Coins(u32),
//!     Book {name: String},
//!     Knowledge,
//! }
//! ```
//! If later binded to lua
//! ```
//! # use rlua::UserData;
//! # use rlua_builders::LuaBuilder;
//! # use rlua_builders_derive::*;
//! # #[derive(LuaBuilder, UserData, Clone)]
//! # enum Valuables {}
//! # rlua::Lua::new().context::<_, rlua::Result<()>>(|ctx| {
//! ctx.globals().set("Valuables", Valuables::builder(ctx)?)?;
//! # Ok(())
//! # }).unwrap();
//! ```
//! Can be used in a very similar way to Rust
//! ```lua
//! local a = Valuables.Coins(12)
//! local b = Valuables.Knowledge
//! local c = Valuables.Book { name = "A Dance with Dragons" }
//! ```
//!
//! [`UserData`]: rlua::UserData
//! [`rlua-builders-derive`]: https://crates.io/crates/rlua-builders-derive
//! [unit structs]: http://doc.rust-lang.org/1.43.1/book/ch05-01-defining-structs.html#unit-like-structs-without-any-fields
//! [tuple structs]: https://doc.rust-lang.org/1.43.1/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types
//! [normal structs]: https://doc.rust-lang.org/1.43.1/book/ch05-01-defining-structs.html#defining-and-instantiating-structs
//! [enums]: https://doc.rust-lang.org/1.43.1/book/ch06-01-defining-an-enum.html#defining-an-enum
use rlua::{Context, Result, ToLua, UserData};

/// A struct or enum with this Trait provides a Lua builder.
///
/// Should not be implemented manually, instead use the [`rlua-builders-derive`] crate
/// to derive it automatically for any struct or enum.
///
/// [`rlua-builders-derive`]: https://crates.io/crates/rlua-builders-derive
pub trait LuaBuilder<'s, T: ToLua<'s>>: UserData + Clone {
    /// Create a Lua builder for this type
    fn builder(ctx: Context<'s>) -> Result<T>;
}
