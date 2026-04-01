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
            println!("  Total pixels: {}", width * height);
        }
        Err(e) => {
            println!("Failed to open image: {}", e);
        }
    }
}
