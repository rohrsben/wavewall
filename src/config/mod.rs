use std::env;

pub fn config_dir() -> String {
    if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
        format!("{xdg}/wavewall")
    } else {
        let user = env::var("USER").unwrap();
        format!("/home/{user}/.config/wavewall")
    }
}

pub fn config_file() -> String {
    format!("{}/wavewall.lua", config_dir())
}
