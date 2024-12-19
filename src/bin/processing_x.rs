use ab_glyph::{Font, FontRef, PxScaleFont, ScaleFont as _};
use image::{Rgba, RgbaImage};
use imageproc::{
    drawing::{draw_filled_rect_mut, draw_text_mut},
    rect::Rect,
};

const WIDTH: u32 = 300; // 画像の幅
const HEIGHT: u32 = 300; // 画像の高さ
const OUTPUT_DIR: &str = "output/"; // 画像の保存先
const FONT_SIZE: f32 = 64.0 * 72.0 / 96.0; // フォントサイズ (96dpi -> 72dpi に変換)
const TEXT: &str = "Hoge"; // テキスト

fn main() {
    // 1. 300 x 300 の黒背景を作成
    let mut image = RgbaImage::new(WIDTH, HEIGHT);

    draw_filled_rect_mut(
        &mut image,
        Rect::at(0, 0).of_size(WIDTH, HEIGHT),
        Rgba([0, 0, 0, 255]),
    );

    // 2. フォントを読み込む
    let font = FontRef::try_from_slice(include_bytes!("../font/DejaVuSans.ttf")).unwrap();

    // 3. 位置計算
    let px_scale = font.pt_to_px_scale(FONT_SIZE).unwrap();
    let px_scale_font = font.as_scaled(px_scale);

    let real_x_pos = calc_x(TEXT, &px_scale_font);
    let real_y_pos = 0.0;

    // 4. テキストを描画
    draw_text_mut(
        &mut image,
        Rgba([255u8, 0u8, 0u8, 255u8]),
        real_x_pos as i32,
        real_y_pos as i32,
        px_scale,
        &font,
        TEXT,
    );

    image
        .save(format!("{}processing_x.png", OUTPUT_DIR))
        .unwrap();
}

// 移動する x 座標を計算
fn calc_x<F: Font>(text: &str, px_scale_font: &PxScaleFont<F>) -> f32 {
    let first_char = text.chars().next().unwrap();
    let h_side_beraring = px_scale_font.h_side_bearing(px_scale_font.glyph_id(first_char));
    -h_side_beraring
}
