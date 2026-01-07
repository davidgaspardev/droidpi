use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

/// Clips an image to a circular shape, automatically centering and using max radius
///
/// This function creates a circular mask that makes all pixels outside the circle
/// transparent, preserving the original pixels inside the circle.
///
/// # Arguments
/// * `img` - The input image to clip
///
/// # Returns
/// A new RGBA image with circular clipping applied (transparent outside the circle)
pub fn clip_to_circle(img: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let radius = center_x.min(center_y);

    let mut output = ImageBuffer::new(width, height);
    let radius_squared = radius.powf(2.0);

    for y in 0..height {
        for x in 0..width {
            // Calculate distance from center
            let dx_squared = (x as f32 - center_x).powf(2.0);
            let dy_squared = (y as f32 - center_y).powf(2.0);
            let distance_squared = dx_squared + dy_squared;

            if distance_squared <= radius_squared {
                let pixel = img.get_pixel(x, y);
                output.put_pixel(x, y, pixel);
            } else {
                output.put_pixel(x, y, Rgba([0, 0, 0, 0]));
            }
        }
    }

    output
}
