pub mod tests {
    #![allow(dead_code)]
    use mlua::{Lua, Value};

    pub struct TestVal {
        lua: Lua,
        pub val: Value
    }

    impl TestVal {
        pub fn new(input: &'static str) -> Self {
            let lua = Lua::new();
            let val = lua.load(input).eval::<Value>().unwrap();

            Self { lua, val }
        }

        pub fn new_with_lua(lua: Lua, input: &'static str) -> Self {
            let val = lua.load(input).eval::<Value>().unwrap();

            Self { lua, val }
        }
    }
}

#[macro_export]
macro_rules! opt_simple {
    ($name: ident, $type: ident, $table: ident, $loc: ident) => {
        let $name = match crate::config::parse::$type(
            $table.get::<Value>(stringify!($name))?
        ) {
            Ok(val) => val,
            Err(info) => {
                let location = $loc.add_parent(stringify!($name));
                return Err(info.with_location(location).into())
            }
        };
    }
}

#[macro_export]
macro_rules! opt_complex {
    ($name: ident, $table: ident, $loc: ident) => {
        let $name = $name::parse(
            $table.get::<Value>(stringify!($name))?,
            &$loc
        )?;
    };
}
