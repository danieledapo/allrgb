use std::collections::HashSet;

pub mod image;

pub use image::{Image, Rgb};

pub fn generate(
    mut colors: Vec<Rgb>,
    (width, height): (usize, usize),
    (sx, sy): (usize, usize),
) -> Image<Rgb> {
    assert!(colors.len() == width * height);

    let mut img = Image::new((0, 0, 0), width, height);
    let mut seen = Image::new(false, width, height);

    img.set(sx, sy, colors.pop().unwrap());
    seen.set(sx, sy, true);

    let mut free = HashSet::new();
    img.for_each_neighbor(sx, sy, |x, y| {
        free.insert((x, y));
    });

    while let Some(rgb) = colors.pop() {
        let &(x, y) = free
            .iter()
            .min_by_key(|&&(x, y)| {
                let mut neighbors = 0;
                let mut total_color_dist = 0;

                img.for_each_neighbor(x, y, |xx, yy| {
                    if !seen.get(xx, yy) {
                        return;
                    }

                    neighbors += 1;
                    total_color_dist += color_dist(rgb, img.get(xx, yy));
                });

                debug_assert!(neighbors > 0);
                total_color_dist / neighbors
            })
            .unwrap();

        debug_assert!(!seen.get(x, y));
        seen.set(x, y, true);

        free.remove(&(x, y));
        img.for_each_neighbor(x, y, |nx, ny| {
            if !seen.get(nx, ny) {
                free.insert((nx, ny));
            }
        });

        img.set(x, y, rgb);
    }

    img
}

fn color_dist((r1, g1, b1): Rgb, (r2, g2, b2): Rgb) -> i32 {
    let dr = i32::from(r1) - i32::from(r2);
    let dg = i32::from(g1) - i32::from(g2);
    let db = i32::from(b1) - i32::from(b2);

    dr * dr + dg * dg + db * db
}
