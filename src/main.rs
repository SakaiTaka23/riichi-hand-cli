mod cli;
mod image_handler;

use clap::Parser;
use image::RgbaImage;
use image_handler::{save_as_file, save_to_clipboard};
use riichi_hand::parser::HandParser;
use riichi_hand::raster_renderer::fluffy_stuff_tile_sets::{
    BLACK_FLUFFY_STUFF_TILE_SET, RED_FLUFFY_STUFF_TILE_SET, YELLOW_FLUFFY_STUFF_TILE_SET,
};
use riichi_hand::raster_renderer::martin_persson_tile_sets::MARTIN_PERSSON_TILE_SET;
use riichi_hand::raster_renderer::{RasterRenderer, RenderOptions};
use riichi_hand::Hand;

fn main() {
    let args = cli::Args::parse();
    let cli::Args { hand, name, tile } = args;
    let options = RenderOptions::default();

    let hand = HandParser::parse(&*hand).unwrap();

    process_hand(&hand, name, tile, options).unwrap();
}

fn process_hand(
    hand: &Hand,
    name: Option<String>,
    tile: String,
    options: RenderOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    let image = match tile.as_str() {
        "yellow" => RasterRenderer::render(hand, &*YELLOW_FLUFFY_STUFF_TILE_SET, options),
        "red" => RasterRenderer::render(hand, &*RED_FLUFFY_STUFF_TILE_SET, options),
        "black" => RasterRenderer::render(hand, &*BLACK_FLUFFY_STUFF_TILE_SET, options),
        "martin" => RasterRenderer::render(hand, &*MARTIN_PERSSON_TILE_SET, options),
        _ => panic!("invalid tile set {}", tile),
    }
    .unwrap();
    let rgba: RgbaImage = image.into();

    if name.is_none() {
        save_to_clipboard(rgba).unwrap();
    } else {
        save_as_file(rgba, name.unwrap()).unwrap();
    };
    Ok(())
}
