mod kdl_util;

use std::{env, fs};
use kdl::KdlDocument as KDoc;
use kdl::KdlNode as KNode;

#[derive(Debug)]
pub struct Config {
    pub general: Option<General>
}

#[derive(Debug)]
pub struct General {
    pub order: Option<Order>,
    pub tileset: Option<String>
}

#[derive(Debug)]
pub enum Order {
    Random,
    Set
}

pub fn load() -> Config {
    let parsed = get_conf();

    Config {
        general: parse_general(parsed.get("general"))
    }
}

fn parse_general(node: Option<&KNode>) -> Option<General> {
    let node = match node {
        Some(node) => node,
        None => return None
    };

    let children = match node.children() {
        Some(c) => c,
        None => return None
    };

    let order = parse_order(children.get("order"));

    let tileset = parse_tileset(children.get("tileset"));

    Some(General {
        order,
        tileset
    })
}

fn parse_tileset(node: Option<&KNode>) -> Option<String> {
    let entries = match node {
        Some(node) => node.entries(),
        None => return None
    };

    let res = match entries.len() {
        0 => return None,
        _ => &entries[0]
    };

    let val = res.value();

    if val.is_string() {
        match val.as_string() {
            Some(s) => Some(s.to_owned()),
            None => None
        }
    } else {
        return None
    }
}

fn parse_order(node: Option<&KNode>) -> Option<Order> {
    let entries = match node {
        Some(node) => node.entries(),
        None => return None
    };

    let res = match entries.len() {
        0 => return None,
        _ => &entries[0]
    };

    let val = res.value();

    if val.is_string() {
        match val.as_string() {
            Some("random") => Some(Order::Random),
            Some("set") => Some(Order::Set),
            _ => None
        }
    } else {
        None
    }
}

fn get_conf() -> KDoc {
    let conf = match fs::read_to_string(config_file()) {
        Ok(result) => result,
        Err(_) => String::new()
    };

    match conf.parse() {
        Ok(doc) => doc,
        Err(e) => panic!("fucked up the config: {}", e)
    }
}

fn config_file() -> String {
    let mut dir = config_dir();
    dir.push_str("/wavewall.kdl");

    dir
}

pub fn config_dir() -> String {
    if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
        format!("{xdg}/wavewall")
    } else {
        let user = env::var("USER").unwrap();
        format!("/home/{user}/.config/wavewall")
    }
}
