use rlua::{Context, Result, Table, ToLua};

pub trait LuaStructBuilder<'s, T: ToLua<'s>> {
    fn builder(ctx: Context<'s>) -> Result<T>;
}

pub trait LuaEnumBuilder<'s> {
    fn builder(ctx: Context<'s>) -> Result<Table<'s>>;
}
