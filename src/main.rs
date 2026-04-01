mod encryption;
mod images;

use image::GenericImageView;

const KEY: &[u8; 16] = b"mysecretkey12345";
const IV: &[u8; 16] = b"randomiv12345678";

struct Demo {
    name: &'static str,
    input: &'static str,
    ecb_out: &'static str,
    cbc_out: &'static str,
}

const DEMOS: &[Demo] = &[
    Demo { name: "synthetic", input: "assets/synthetic.png",  ecb_out: "assets/synthetic_ecb.png",  cbc_out: "assets/synthetic_cbc.png"  },
    Demo { name: "tux",       input: "assets/tux.png",        ecb_out: "assets/tux_ecb.png",        cbc_out: "assets/tux_cbc.png"        },
    Demo { name: "canada",    input: "assets/canada.png",     ecb_out: "assets/canada_ecb.png",     cbc_out: "assets/canada_cbc.png"     },
    Demo { name: "brazil",    input: "assets/brazil.png",     ecb_out: "assets/brazil_ecb.png",     cbc_out: "assets/brazil_cbc.png"     },
];

fn process(demo: &Demo) {
    if demo.name == "synthetic" {
        let img = images::generate_test_image(512, 512);
        img.save(demo.input).expect("Failed to save synthetic image");
    }

    let img = image::open(demo.input)
        .unwrap_or_else(|_| panic!("Could not open {}", demo.input));

    let (width, height) = img.dimensions();
    let bytes = img.to_rgb8();
    let raw = bytes.as_raw();

    let ecb = encryption::encrypt_ecb(raw, KEY);
    images::apply_encrypted(&img, &ecb[..raw.len()])
        .save(demo.ecb_out)
        .expect("Failed to save ECB output");

    let cbc = encryption::encrypt_cbc(raw, KEY, IV);
    images::apply_encrypted(&img, &cbc[..raw.len()])
        .save(demo.cbc_out)
        .expect("Failed to save CBC output");

    println!("{} ({}x{}): done", demo.name, width, height);
    println!("  ECB -> {}", demo.ecb_out);
    println!("  CBC -> {}", demo.cbc_out);
}

fn main() {
    for demo in DEMOS {
        process(demo);
    }
}
