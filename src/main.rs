use image::GenericImageView;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run -- <path-to-image>");
        return;
    }

    let path = &args[1];

    match image::open(path) {
        Ok(img) => {
            let (width, height) = img.dimensions();
            println!("Image: {}", path);
            println!("  Size: {}x{} pixels", width, height);
            println!("  Color type: {:?}", img.color());

            let rgb = img.to_rgb8();

            let bytes: &[u8] = rgb.as_raw();

            println!("  Raw byte count: {}", bytes.len());
            println!("  (should be {}x{}x3 = {})", width, height, width * height * 3);

            println!("\nFirst 3 pixels (R,G,B):");
            for i in 0..3 {
                let r = bytes[i * 3];
                let g = bytes[i * 3 + 1];
                let b = bytes[i * 3 + 2];
                println!("  Pixel {}: R={:3} G={:3} B={:3}", i, r, g, b);
            }
        }
        Err(e) => {
            println!("Failed to open image: {}", e);
        }
    }
}
