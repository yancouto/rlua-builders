# rlua-builders

![Travis (.org)](https://img.shields.io/travis/yancouto/rlua-builders?style=for-the-badge)
![Crates.io](https://img.shields.io/crates/v/rlua-builders?style=for-the-badge)

This package allows Rust structs/enums to be easily created from Lua.

```rust
enum Valuables {
    Coins(u32),
    Book {name: String},
    Knowledge,
}
```

Can then be created from Lua as:
```lua
local a = Valuables.Coins(12)
local b = Valuables.Knowledge
local c = Valuables.Book { name = "A Dance with Dragons" }
```

See [the documentation](https://docs.rs/rlua-builders/*/rlua_builders/) for more information.