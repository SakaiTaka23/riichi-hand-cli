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
use rustyline::{Config, DefaultEditor, EditMode};

use crate::image_handler::{save_as_file, save_to_clipboard};

pub fn process_hand(
    hand: &str,
    name: &Option<String>,
    tile: &str,
    options: RenderOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    let hand = &HandParser::parse(hand)?;
    let image = match tile {
        "yellow" => RasterRenderer::render(hand, &*YELLOW_FLUFFY_STUFF_TILE_SET, options),
        "red" => RasterRenderer::render(hand, &*RED_FLUFFY_STUFF_TILE_SET, options),
        "black" => RasterRenderer::render(hand, &*BLACK_FLUFFY_STUFF_TILE_SET, options),
        "martin" => RasterRenderer::render(hand, &*MARTIN_PERSSON_TILE_SET, options),
        _ => panic!("invalid tile set {}", tile),
    }
    .unwrap();
    let rgba: RgbaImage = image.into();

    match name {
        Some(name) => save_as_file(rgba, name)?,
        None => save_to_clipboard(rgba)?,
    }

    Ok(())
}

pub fn interactive_mode(name: &Option<String>, tile: &str, options: RenderOptions) {
    let config = Config::builder().edit_mode(EditMode::Vi).build();
    let mut rl = DefaultEditor::with_config(config).unwrap();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let hand = line.trim();
                if hand == "exit" || hand.is_empty() {
                    break;
                }
                if let Err(e) = rl.add_history_entry(line.as_str()) {
                    println!("Error: {}", e);
                }

                if let Err(e) = process_hand(&hand, name, tile, options) {
                    println!("Error: {}", e);
                }
            }
            Err(_) => break,
        }
    }
}
