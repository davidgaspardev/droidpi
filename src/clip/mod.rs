use image::{DynamicImage, Rgba};

pub fn clip_to_circle(img: &DynamicImage) -> image::RgbaImage {
    let mut output = img.to_rgba8();
    let (width, height) = output.dimensions();
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let radius_squared = center_x.min(center_y).powi(2);

    let dx_squared: Vec<f32> = (0..width)
        .map(|x| (x as f32 - center_x).powi(2))
        .collect();

    for (x, y, pixel) in output.enumerate_pixels_mut() {
        if dx_squared[x as usize] + (y as f32 - center_y).powi(2) > radius_squared {
            *pixel = Rgba([0, 0, 0, 0]);
        }
    }

    output
}
