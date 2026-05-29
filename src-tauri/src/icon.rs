use image::{ImageBuffer, Rgba};

const DIGIT_PATTERNS: [[[bool; 3]; 5]; 10] = [
    [[true, true, true], [true, false, true], [true, false, true], [true, false, true], [true, true, true]],
    [[false, true, false], [true, true, false], [false, true, false], [false, true, false], [true, true, true]],
    [[true, true, true], [false, false, true], [true, true, true], [true, false, false], [true, true, true]],
    [[true, true, true], [false, false, true], [true, true, true], [false, false, true], [true, true, true]],
    [[true, false, true], [true, false, true], [true, true, true], [false, false, true], [false, false, true]],
    [[true, true, true], [true, false, false], [true, true, true], [false, false, true], [true, true, true]],
    [[true, true, true], [true, false, false], [true, true, true], [true, false, true], [true, true, true]],
    [[true, true, true], [false, false, true], [false, true, false], [false, true, false], [false, true, false]],
    [[true, true, true], [true, false, true], [true, true, true], [true, false, true], [true, true, true]],
    [[true, true, true], [true, false, true], [true, true, true], [false, false, true], [true, true, true]],
];

const SCALE: u32 = 3;
const DIGIT_W: u32 = 3 * SCALE;
const DIGIT_H: u32 = 5 * SCALE;
const GAP: u32 = 1 * SCALE;
const PAD_X: u32 = 3;
const PAD_Y: u32 = 2;

pub fn generate_date_icon(day: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let digits: Vec<u8> = if day < 10 {
        vec![day as u8]
    } else {
        vec![(day / 10) as u8, (day % 10) as u8]
    };

    let text_w = digits.len() as u32 * DIGIT_W + (digits.len().saturating_sub(1) as u32) * GAP;
    let img_w = text_w + PAD_X * 2;
    let img_h = DIGIT_H + PAD_Y * 2;

    let mut img = ImageBuffer::from_pixel(img_w, img_h, Rgba([0, 0, 0, 0]));

    let mut x_offset = PAD_X;
    for &d in &digits {
        let pattern = &DIGIT_PATTERNS[d as usize];
        for (row, pixel_row) in pattern.iter().enumerate() {
            for (col, &on) in pixel_row.iter().enumerate() {
                if on {
                    for sy in 0..SCALE {
                        for sx in 0..SCALE {
                            let px = x_offset + col as u32 * SCALE + sx;
                            let py = PAD_Y + row as u32 * SCALE + sy;
                            if px < img_w && py < img_h {
                                img.put_pixel(px, py, Rgba([0, 0, 0, 255]));
                            }
                        }
                    }
                }
            }
        }
        x_offset += DIGIT_W + GAP;
    }

    img
}

pub fn icon_to_tauri_image(img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> tauri::image::Image<'static> {
    tauri::image::Image::new_owned(img.as_raw().clone(), img.width(), img.height())
}
