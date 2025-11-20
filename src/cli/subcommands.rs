use super::Args;
use crate::image::Image;
use crate::config;
use crate::AppError;
use crate::runtime::get_tiles;

use hex_color::HexColor;
use clap::Subcommand;
use colored::Colorize;

use std::collections::{BinaryHeap, HashMap};

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    /// Generates a tile template
    Template {
        /// The length of the template image's sides
        length: usize
    },

    /// Outputs the colors used in a tileset in multiple formats, ranked by popularity
    Colors {
        /// the tileset to summarize
        tileset: String
    }
}

impl Subcommands {
    pub fn run(&self, args: &Args) -> Result<(), AppError> {
        match self {
            Subcommands::Template { length } => {
                let template = Image::create_template(*length);

                let path = match &args.path {
                    Some(path) => path.clone(),
                    None => {
                        format!("{}/template-{}.png", config::config_dir(), length)
                    }
                };

                template.save(&path)
            }
            Subcommands::Colors { tileset } => {
                let images = get_tiles(tileset)?.into_values();

                let mut colors_hash: HashMap<HexColor, usize> = HashMap::new();
                for image in images {
                    for pixel in image.pixels {
                        colors_hash.entry(pixel)
                            .and_modify(|pop| *pop += 1)
                            .or_insert(1);
                    }
                }

                let mut max_heap = BinaryHeap::new();
                let color_pops = colors_hash.iter()
                    .map(|(color, pop)| ColorPop { pop: *pop, color: *color});
                for pop in color_pops {
                    max_heap.push(pop);
                }

                while !max_heap.is_empty() {
                    let color = max_heap.pop().unwrap().color;
                    println!("{} {}, rgba({}, {}, {}, {})", "     ".on_truecolor(color.r, color.g, color.b), color.display_rgba(), color.r, color.g, color.b, color.a)
                }

                Ok(())
            }
        }
    }
}

#[derive(Ord, PartialEq, PartialOrd, Eq)]
struct ColorPop {
    pop: usize,
    color: HexColor
}
