use rlua::UserData;
use rlua_builders::*;
use rlua_builders_derive::*;

#[test]
fn test_all() {
    #[derive(LuaStructBuilder, UserData, Clone, PartialEq, Eq, Debug)]
    struct Unit;

    #[derive(LuaStructBuilder, UserData, Clone, PartialEq, Eq, Debug)]
    struct Tup(i32, String);

    #[derive(LuaStructBuilder, UserData, Clone, PartialEq, Eq, Debug)]
    struct Named {
        a: i32,
        b: String,
    };

    let lua = rlua::Lua::new();
    let (u, t, n) = lua
        .context(|ctx| -> rlua::Result<(Unit, Tup, Named)> {
            ctx.load(include_str!("success_tests.lua"))
                .call::<_, rlua::Function>(())?
                .call::<_, (Unit, Tup, Named)>((
                    Unit::builder(ctx)?,
                    Tup::builder(ctx)?,
                    Named::builder(ctx)?,
                ))
        })
        .unwrap();

    assert_eq!(u, Unit);
    assert_eq!(t, Tup(1, "a".to_owned()));
    assert_eq!(
        n,
        Named {
            a: 1,
            b: "a".to_owned()
        }
    );
}
