use image::{Rgba, RgbaImage};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};

const WIDTH: u32 = 300;
const HEIGHT: u32 = 300;
const OUTPUT_DIR: &str = "output/";

fn main() {
    let mut image = RgbaImage::new(WIDTH, HEIGHT);

    draw_filled_rect_mut(
        &mut image,
        Rect::at(0, 0).of_size(WIDTH, HEIGHT),
        Rgba([0, 0, 0, 255]),
    );

    image.save(format!("{}init.png", OUTPUT_DIR)).unwrap();
}
