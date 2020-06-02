use rlua::{ToLua, Table, Context};

pub trait LuaStructBuilder<'s, T: ToLua<'s>> {
    fn builder(ctx: Context<'s>) -> T;
}

pub trait LuaEnumBuilder<'s> {
    fn builder(ctx: Context<'s>) -> Table<'s>;
}