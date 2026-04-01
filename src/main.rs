use image::{GenericImageView, ImageBuffer, Rgb};
use aes::Aes128;
use aes::cipher::{BlockEncrypt, KeyInit, generic_array::GenericArray};

fn encrypt_ecb(data: &[u8], key: &[u8; 16]) -> Vec<u8> {
    let cipher = Aes128::new(GenericArray::from_slice(key));
    let mut encrypted = data.to_vec();

    let remainder = encrypted.len() % 16;
    if remainder != 0 {
        encrypted.resize(encrypted.len() + (16 - remainder), 0);
    }

    for chunk in encrypted.chunks_mut(16) {
        let block = GenericArray::from_mut_slice(chunk);
        cipher.encrypt_block(block);
    }

    encrypted
}

// Generates a simple flat-color test image: solid background + geometric shapes.
// Flat colors mean large runs of identical 16-byte blocks, which ECB will
// encrypt identically — making the structure visible in the output.
fn generate_test_image(width: u32, height: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    // ImageBuffer::from_fn builds an image pixel by pixel.
    // The closure receives (x, y) and returns an Rgb([r, g, b]) pixel.
    ImageBuffer::from_fn(width, height, |x, y| {
        let cx = width / 2;
        let cy = height / 2;

        // Integer distance squared — avoids floating point
        let dx = (x as i32 - cx as i32).pow(2) as u32;
        let dy = (y as i32 - cy as i32).pow(2) as u32;
        let dist_sq = dx + dy;

        let inner_r = (height / 5).pow(2);
        let outer_r = (height / 3).pow(2);

        if dist_sq < inner_r {
            Rgb([255u8, 200u8, 0u8])    // yellow core
        } else if dist_sq < outer_r {
            Rgb([30u8, 30u8, 180u8])    // blue ring
        } else if y > height * 2 / 3 {
            Rgb([20u8, 120u8, 20u8])    // green bottom band
        } else {
            Rgb([240u8, 240u8, 240u8])  // light grey background
        }
    })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // If no argument given, generate the test image first
    let path = if args.len() < 2 {
        let test_path = "assets/test_input.png";
        println!("No image provided — generating test image at {}", test_path);
        let img = generate_test_image(512, 512);
        img.save(test_path).expect("Failed to save test image");
        test_path.to_string()
    } else {
        args[1].clone()
    };

    match image::open(&path) {
        Ok(img) => {
            let (width, height) = img.dimensions();
            println!("Image: {}  ({}x{})", path, width, height);

            let rgb = img.to_rgb8();
            let bytes: &[u8] = rgb.as_raw();
            let key: [u8; 16] = *b"mysecretkey12345";

            println!("Encrypting {} bytes with AES-ECB...", bytes.len());
            let encrypted = encrypt_ecb(bytes, &key);
            let encrypted = &encrypted[..bytes.len()];

            let out = ImageBuffer::<Rgb<u8>, _>::from_raw(width, height, encrypted.to_vec())
                .expect("Buffer too small");

            let out_path = "assets/encrypted.png";
            out.save(out_path).expect("Failed to save image");
            println!("Saved encrypted image to: {}", out_path);
            println!("Compare assets/test_input.png vs assets/encrypted.png");
        }
        Err(e) => println!("Failed to open image: {}", e),
    }
}
