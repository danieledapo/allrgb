use rand::prelude::*;

pub use allrgb::{Image, Rgb};

fn main() -> std::io::Result<()> {
    let num_colors: usize = 256;
    let width = 4096;
    let height = 4096;

    assert!(num_colors.pow(3) == width * height);

    let mut colors = allrgb::generate_equally_spaced_rgb_colors(num_colors);
    colors.shuffle(&mut thread_rng());

    let mut seeds = std::collections::HashSet::new();
    seeds.insert((width / 2, height / 2));

    let start_ts = std::time::Instant::now();
    let img = allrgb::generate(colors, (width, height), seeds);
    println!("generation took {} secs", start_ts.elapsed().as_secs());

    let f = std::fs::File::create("img4096.ppm")?;
    img.dump_ppm(f)?;

    Ok(())
}
