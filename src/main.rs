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

    process_hand(&hand, &name, &tile, options).unwrap();
    if interactive {
        interactive_mode(&name, &tile, options);
    }
}
