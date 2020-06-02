use rlua::{Context, Result, ToLua};

pub trait LuaBuilder<'s, T: ToLua<'s>> {
    fn builder(ctx: Context<'s>) -> Result<T>;
}
