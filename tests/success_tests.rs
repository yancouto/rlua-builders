use rlua::*;
use rlua_builders::*;
use rlua_builders_derive::*;

#[test]
fn test_struct_unit() {
    #[derive(LuaStructBuilder, UserData)]
    struct Unit;

    let lua = Lua::new();

    lua.context(|ctx| {
        ctx.load(include_str!("success_tests.lua"))
            .eval::<Function>()?
            .call::<_, ()>(Unit::builder(ctx))
    }).unwrap();
}
