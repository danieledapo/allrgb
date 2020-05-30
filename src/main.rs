use std::convert::TryFrom;

use rand::prelude::*;

pub use allrgb::{Image, Rgb};

fn main() -> std::io::Result<()> {
    let num_colors: usize = 32;
    let width = 256;
    let height = 128;

    // let num_colors: usize = 64;
    // let width = 512;
    // let height = 512;

    assert!(num_colors.pow(3) == width * height);

    let mut colors = Vec::with_capacity(num_colors.pow(3));
    for ri in 0..num_colors {
        for gi in 0..num_colors {
            for bi in 0..num_colors {
                colors.push((
                    u8::try_from(ri * 255 / num_colors).unwrap(),
                    u8::try_from(gi * 255 / num_colors).unwrap(),
                    u8::try_from(bi * 255 / num_colors).unwrap(),
                ));
            }
        }
    }
    colors.shuffle(&mut thread_rng());

    let start_ts = std::time::Instant::now();
    let img = allrgb::generate(colors, (width, height), (width / 2, height / 2));
    println!("generation took {} secs", start_ts.elapsed().as_secs());

    let f = std::fs::File::create("img.ppm")?;
    img.dump_ppm(f)?;

    Ok(())
}
