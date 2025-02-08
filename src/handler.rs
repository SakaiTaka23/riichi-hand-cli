use std::io::{self, Write};

use image::RgbaImage;
use riichi_hand::{
    parser::HandParser,
    raster_renderer::{
        fluffy_stuff_tile_sets::{
            BLACK_FLUFFY_STUFF_TILE_SET, RED_FLUFFY_STUFF_TILE_SET, YELLOW_FLUFFY_STUFF_TILE_SET,
        },
        martin_persson_tile_sets::MARTIN_PERSSON_TILE_SET,
        RasterRenderer, RenderOptions,
    },
};

use crate::image_handler::{save_as_file, save_to_clipboard};

pub fn process_hand(
    hand: String,
    name: Option<String>,
    tile: String,
    options: RenderOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    let hand = &HandParser::parse(hand.as_str())?;
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
        save_to_clipboard(rgba)?;
    } else {
        save_as_file(rgba, name.unwrap())?;
    };
    Ok(())
}

pub fn interactive_mode(name: Option<String>, tile: String, options: RenderOptions) {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut hand = String::new();
        io::stdin().read_line(&mut hand).unwrap();
        let hand = hand.trim();
        if hand == "exit" || hand.is_empty() {
            break;
        }

        if let Err(e) = process_hand(hand.to_string(), name.clone(), tile.clone(), options) {
            println!("Error: {}", e);
        }
    }
}
