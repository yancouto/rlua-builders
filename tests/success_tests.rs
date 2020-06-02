use rlua::UserData;
use rlua_builders::*;
use rlua_builders_derive::*;

#[test]
fn test_all() {
    #[derive(LuaStructBuilder, UserData, Clone, PartialEq, Debug)]
    struct Unit;

    #[derive(LuaStructBuilder, UserData, Clone, PartialEq, Debug)]
    struct Tup(i32, String);

    #[derive(LuaStructBuilder, UserData, Clone, PartialEq, Debug)]
    struct Named {
        a: i32,
        b: String,
    };

    #[derive(LuaEnumBuilder, UserData, Clone, PartialEq, Debug)]
    enum ComplexEnum {
        Unit,
        Tup(f32, Option<i32>),
        Named { foo: String, bar: u8 },
        // also testing tuples with a single thing inside
        Composite(Tup),
    };

    let lua = rlua::Lua::new();
    let (u, t, n, ce) = lua
        .context(
            |ctx| -> rlua::Result<(Unit, Tup, Named, Vec<ComplexEnum>)> {
                ctx.load(include_str!("success_tests.lua"))
                    .call::<_, rlua::Function>(())?
                    .call::<_, _>((
                        Unit::builder(ctx)?,
                        Tup::builder(ctx)?,
                        Named::builder(ctx)?,
                        ComplexEnum::builder(ctx)?,
                    ))
            },
        )
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
    assert_eq!(
        ce,
        vec![
            ComplexEnum::Unit,
            ComplexEnum::Tup(0.1, Some(42)),
            ComplexEnum::Named {
                foo: "baz".to_owned(),
                bar: 0
            },
            ComplexEnum::Composite(Tup(0, "zero".to_owned())),
        ]
    );
}
