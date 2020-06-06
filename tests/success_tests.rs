use rlua::UserData;
use rlua_builders::*;
use rlua_builders_derive::*;

#[test]
fn test_all() {
    #[derive(LuaBuilder, UserData, Clone, PartialEq, Debug)]
    struct Unit;

    #[derive(LuaBuilder, UserData, Clone, PartialEq, Debug)]
    struct Tup(i32, String);

    #[derive(LuaBuilder, UserData, Clone, PartialEq, Debug)]
    struct Named {
        a: i32,
        b: String,
    };

    #[derive(LuaBuilder, UserData, Clone, PartialEq, Debug)]
    enum ComplexEnum {
        Unit,
        Tup(f32, Option<i32>),
        Named {
            foo: String,
            #[default = 10]
            baz: u8,
        },
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
                foo: "bar".to_owned(),
                baz: 10,
            },
            ComplexEnum::Composite(Tup(0, "zero".to_owned())),
        ]
    );
}

#[test]
fn test_user_data() {
    #[derive(Debug, Clone)]
    struct B;

    #[derive(Debug, Clone, UserData)]
    struct A(B);
}
