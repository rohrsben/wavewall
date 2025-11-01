#[derive(Debug)]
pub struct Wavewall;

impl mlua::UserData for Wavewall {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("rgb_to_hex", |_, this| {
            
        });
    }
}
