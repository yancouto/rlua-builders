use rlua::UserData;
use rlua_builders::*;
use rlua_builders_derive::*;

#[test]
fn test_all() {
    #[derive(LuaStructBuilder, UserData)]
    struct Unit;

    #[derive(LuaStructBuilder, UserData)]
    struct Tup(i32, String);

    let lua = rlua::Lua::new();

    lua.context(|ctx| {
        ctx.load(include_str!("success_tests.lua"))
            .call::<_, rlua::Function>(())?
            .call::<_, ()>((Unit::builder(ctx)?, Tup::builder(ctx)?))
    })
    .unwrap();
}
