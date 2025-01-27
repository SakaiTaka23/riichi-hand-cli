use std::borrow::Cow;
use std::path::Path;

use arboard::Clipboard;
use clap::Parser;
use image::RgbaImage;
use riichi_hand::parser::HandParser;
use riichi_hand::raster_renderer::fluffy_stuff_tile_sets::{
    BLACK_FLUFFY_STUFF_TILE_SET, RED_FLUFFY_STUFF_TILE_SET, YELLOW_FLUFFY_STUFF_TILE_SET,
};
use riichi_hand::raster_renderer::martin_persson_tile_sets::MARTIN_PERSSON_TILE_SET;
use riichi_hand::raster_renderer::{RasterRenderer, RenderOptions};

const LONG_ABOUT: &str = "
Hands need to be represented in a text format. The format is described below.
    - Numerical tiles consist of a digit and a letter representing the tile suite (m, s, p for manzu, souzu, and pinzu, respectively).
    - Zero means a red five.
    - Examples: 1s (1 of bamboos), 5p (5 of circles), 3m (3 of characters), 0s (red 5 of bamboos).
    - Honor tiles are represented using the tile suite z. Tiles 1-4 are wind tiles (East, South, West, North), and 5-7 are dragon tiles (White, Green, Red).
    - Examples: 1z (East), 6z (green dragon).
    - For convenience, single-character format is also available. E, S, W, N is East, South, West, and North, respectively, while w, g, r are white, green, and red dragon, respectively.
    - To get a reversed tile, ? can be used. Example: ?33m? (closed kan of 3 of characters).
    - In case of longer sequences of tiles in the same suite, the tile suite characters can be omitted except for the last one. Example: 123s is the same as 1s2s3s.
    - Tiles can be rotated by putting an asterisk () after the tile, or rotated and shifted with double asterisk (**). Examples: 1s (rotated 1 of bamboos), 33**33p (open kan of 3 of circles).
    - Groups of tiles can be separated using an underscore (_). Example: 123s_456s.

More examples:
    - 123s345m345m222pWW
    - EE_www_ggg_rrr*_?WW?
    - 22m11s33s77pEEggrr
";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = LONG_ABOUT)]
struct Args {
    /// Mahjong hand in human readable format
    /// --help for more information
    hand: String,

    /// Name and path of the image to save.
    /// If not specified, the image will be copied to clipboard
    #[arg(short, long)]
    name: Option<String>,

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
    }
    .unwrap();

    if name.is_none() {
        let mut clipboard = Clipboard::new().unwrap();
        let rgba: RgbaImage = image.into();
        clipboard
            .set_image(arboard::ImageData {
                width: rgba.width() as usize,
                height: rgba.height() as usize,
                bytes: Cow::Owned(rgba.as_raw().to_vec()),
            })
            .unwrap();
    } else {
        image.save(get_available_filename(&name.unwrap())).unwrap();
    };
}

fn get_available_filename(original_path: &String) -> String {
    if !Path::new(original_path).exists() {
        return original_path.to_string();
    }

    let path = Path::new(original_path);
    let stem = path.file_stem().unwrap().to_str().unwrap();
    let ext = path.extension().unwrap().to_str().unwrap();

    let mut counter = 1;
    loop {
        let new_name = format!("{}({}).{}", stem, counter, ext);
        if !Path::new(&new_name).exists() {
            return new_name;
        }
        counter += 1;
    }
}
