use clap::Parser;
use riichi_hand::parser::HandParser;
use riichi_hand::raster_renderer::fluffy_stuff_tile_sets::{
    BLACK_FLUFFY_STUFF_TILE_SET, RED_FLUFFY_STUFF_TILE_SET, YELLOW_FLUFFY_STUFF_TILE_SET,
};
use riichi_hand::raster_renderer::martin_persson_tile_sets::MARTIN_PERSSON_TILE_SET;
use riichi_hand::raster_renderer::{RasterRenderer, RenderOptions};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    hand: String,

    /// Name and path of the image to save
    #[arg(short, long, default_value = "hand.png")]
    name: String,

    /// Tile design to use
    #[arg(short, long, default_value = "yellow")]
    tile: String,
}

fn main() {
    let args = Args::parse();
    let Args { hand, name, tile } = args;

    let hand = HandParser::parse(&*hand).unwrap();
    let options = RenderOptions::default();

    let image = match tile.as_str() {
        "yellow" => RasterRenderer::render(&hand, &*YELLOW_FLUFFY_STUFF_TILE_SET, options),
        "red" => RasterRenderer::render(&hand, &*RED_FLUFFY_STUFF_TILE_SET, options),
        "black" => RasterRenderer::render(&hand, &*BLACK_FLUFFY_STUFF_TILE_SET, options),
        "martin" => RasterRenderer::render(&hand, &*MARTIN_PERSSON_TILE_SET, options),
        _ => panic!("invalid tile set {}", tile),
    };

    image.unwrap().save(name).unwrap();
}
