extern crate docopt;
extern crate image;
extern crate rand;

use docopt::Docopt;
use std::path::Path;
use image::{
    RgbImage,
    Rgb,
    imageops
};
use rand::ThreadRng;
use rand::distributions::{
    IndependentSample,
    Range
};

const TILE_SIZE : u32 = 5;

const TILES: [ [Rgb<u8>; 3]; 5] = [
[ Rgb{data: [31, 31, 31]}, Rgb{data: [112, 112, 122]}, Rgb{data: [92, 92, 92]}],
[ Rgb{data: [31, 31, 31]}, Rgb{data: [ 95,  95,  95]}, Rgb{data: [79, 79, 79]}],
[ Rgb{data: [31, 31, 31]}, Rgb{data: [ 85,  85,  85]}, Rgb{data: [71, 71, 71]}],
[ Rgb{data: [31, 31, 31]}, Rgb{data: [ 63,  63,  63]}, Rgb{data: [55, 55, 55]}],
[ Rgb{data: [31, 31, 31]}, Rgb{data: [ 49,  49,  49]}, Rgb{data: [44, 44, 44]}]
];


fn draw_tile(img: &mut RgbImage, x: u32, y: u32, tile: &[Rgb<u8>;3]) {
    img.put_pixel(x, y, tile[0]);
    img.put_pixel(x, y+1, tile[0]);
    img.put_pixel(x, y+2, tile[0]);
    img.put_pixel(x, y+3, tile[0]);
    img.put_pixel(x, y+4, tile[0]);
    img.put_pixel(x+1, y, tile[0]);
    img.put_pixel(x+2, y, tile[0]);
    img.put_pixel(x+3, y, tile[0]);
    img.put_pixel(x+4, y, tile[0]);

    img.put_pixel(x+1, y+1, tile[1]);
    img.put_pixel(x+1, y+2, tile[1]);
    img.put_pixel(x+1, y+3, tile[1]);
    img.put_pixel(x+1, y+4, tile[1]);
    img.put_pixel(x+2, y+1, tile[1]);
    img.put_pixel(x+3, y+1, tile[1]);
    img.put_pixel(x+4, y+1, tile[1]);
    img.put_pixel(x+2, y+4, tile[1]);
    img.put_pixel(x+3, y+4, tile[1]);
    img.put_pixel(x+4, y+2, tile[1]);
    img.put_pixel(x+4, y+3, tile[1]);
    img.put_pixel(x+4, y+4, tile[1]);

    img.put_pixel(x+2, y+2, tile[1]);
    img.put_pixel(x+2, y+3, tile[1]);
    img.put_pixel(x+3, y+2, tile[1]);
    img.put_pixel(x+3, y+3, tile[1]);
}

fn draw_tile_rand(img: &mut RgbImage, x: u32, y: u32) {
    let between : Range<u8> = Range::new(0, TILES.len() as u8);
    let mut rng : ThreadRng = rand::thread_rng();
    let idx = between.ind_sample(&mut rng) as usize;
    draw_tile(img, x, y, &TILES[idx]);
}

fn generate_image<F>(path: &Path, orig_width: u32, orig_height: u32, draw_image: F)
where F: Fn(&mut RgbImage, u32, u32) {

    let width = ((orig_width + TILE_SIZE -1 ) / TILE_SIZE) * TILE_SIZE;
    let height = ((orig_height + TILE_SIZE -1 ) / TILE_SIZE) * TILE_SIZE;

    let mut img = RgbImage::new(width, height);

    draw_image(&mut img, width, height);

    let subimg;
    let imgbuf;
    let imgref;
    if width != orig_width || height != orig_height {
        subimg = imageops::crop(&mut img, 0, 0, orig_width, orig_height);
        imgbuf = subimg.to_image();
        imgref = &imgbuf;
    } else {
        imgref = &img;
    }
    let _ = imgref.save(path);
}

fn generate_mosaic(path: &Path, orig_width: u32, orig_height: u32) {
    generate_image(path, orig_width, orig_height,
       |img: &mut RgbImage, width: u32, height: u32| {
           for y in 0..(height/TILE_SIZE) {
               for x in 0..(width/TILE_SIZE) {
                   draw_tile_rand(img, x*TILE_SIZE ,y*TILE_SIZE);
               }
           }
       });
}

const USAGE: &'static str = "
Mosaic background generator.

Usage: mosaic [options] FILE
       mosaic -g GEOM <kind> FILE
       mosaic --geometry GEOM <kind> FILE
       mosaic -h | --help
       mosaic -v | --version

Options:
    -h, --help                            Show this message
    -v, --version                         Show the version
    -g=<WIDTHxHEIGHT>, --geometry=<WIDTHxHEIGHT>  Geometry of the image to generate [default: 100x100]
";



fn geometry_parse(geometry: &str) -> (u32, u32) {
    let geometry : Vec<&str> = geometry.split('x').collect();
    if geometry.len() != 2 {
        panic!("invalid geometry");
    }
    let width : u32 = geometry[0].parse()
        .ok()
        .expect("invalid geometry");
    let height : u32 = geometry[1].parse()
        .ok()
        .expect("invalid geometry");
    (width, height)
}

fn main() {
    let version = env!("CARGO_PKG_VERSION").to_owned();
    let args = Docopt::new(USAGE)
                      .and_then(|dopt| dopt.version(Some(version)).parse())
                      .unwrap_or_else(|e| e.exit());
    let geometry = args.get_str("--geometry");
    let geometry = geometry_parse(&geometry);

    let path = args.get_str("FILE");
    let path = Path::new(path);

    generate_mosaic(path, geometry.0, geometry.1);
}