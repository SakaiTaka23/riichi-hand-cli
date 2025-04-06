mod cli;
mod handler;
mod image_handler;

use clap::Parser;
use handler::{interactive_mode, process_hand};
use riichi_hand::raster_renderer::RenderOptions;

fn main() {
    let args = cli::Args::parse();
    let cli::Args {
        hand,
        interactive,
        name,
        tile,
    } = args;
    let options = RenderOptions::default();

    if let Some(h) = &hand {
        process_hand(&h, &name, &tile, options).unwrap();
    }
    if hand.is_none() || interactive {
        interactive_mode(&name, &tile, options);
    }
}
