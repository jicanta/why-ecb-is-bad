use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};

pub fn generate_test_image(width: u32, height: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    ImageBuffer::from_fn(width, height, |x, y| {
        let cx = width / 2;
        let cy = height / 2;
        let dx = (x as i32 - cx as i32).pow(2) as u32;
        let dy = (y as i32 - cy as i32).pow(2) as u32;
        let dist_sq = dx + dy;
        let inner_r = (height / 5).pow(2);
        let outer_r = (height / 3).pow(2);

        if dist_sq < inner_r {
            Rgb([255u8, 200u8, 0u8])
        } else if dist_sq < outer_r {
            Rgb([30u8, 30u8, 180u8])
        } else if y > height * 2 / 3 {
            Rgb([20u8, 120u8, 20u8])
        } else {
            Rgb([240u8, 240u8, 240u8])
        }
    })
}

pub fn apply_encrypted(img: &DynamicImage, encrypted_bytes: &[u8]) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    ImageBuffer::<Rgb<u8>, _>::from_raw(width, height, encrypted_bytes.to_vec())
        .expect("Buffer size mismatch")
}
