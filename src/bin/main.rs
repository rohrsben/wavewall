use std::env::set_current_dir;

use libwavewall::config;

fn main() {
    // TODO make dir if nonexistent? populate with default config?
    let _ = set_current_dir(config::config_dir());

    let config = config::generate();

    match config {
        Ok(conf) => println!("{}", conf.output.filepath()),
        Err(e) => println!("{}", e),
    };
}
