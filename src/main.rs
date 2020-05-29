use std::collections::HashSet;
use std::convert::TryFrom;
use std::io::{self, Write};

use rand::prelude::*;

pub type Rgb = (u8, u8, u8);

pub struct Image<Pixel = Rgb> {
    data: Vec<Pixel>,
    width: usize,
    height: usize,
}

fn main() -> std::io::Result<()> {
    let num_colors: usize = 32;
    let width = 256;
    let height = 128;
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

    let mut img = Image::new((0, 0, 0), width, height);
    let mut seen = Image::new(false, width, height);

    let (sx, sy) = (width / 2, height / 2);
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

    let f = std::fs::File::create("img.ppm")?;
    img.dump_ppm(f)?;

    Ok(())
}

impl<Pixel: Clone + Copy> Image<Pixel> {
    pub fn new(pix: Pixel, width: usize, height: usize) -> Self {
        Image {
            width,
            height,
            data: vec![pix; width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Pixel {
        self.data[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, rgb: Pixel) {
        self.data[y * self.width + x] = rgb;
    }

    pub fn for_each_neighbor(&self, x: usize, y: usize, mut fun: impl FnMut(usize, usize)) {
        if x > 0 && y > 0 {
            fun(x - 1, y - 1);
        }
        if x > 0 {
            fun(x - 1, y);
        }
        if x > 0 && y + 1 < self.height {
            fun(x - 1, y + 1);
        }

        if y > 0 {
            fun(x, y - 1);
        }
        if y + 1 < self.height {
            fun(x, y + 1);
        }

        if x + 1 < self.width && y > 0 {
            fun(x + 1, y - 1);
        }
        if x + 1 < self.width {
            fun(x + 1, y);
        }
        if x + 1 < self.width && y + 1 < self.height {
            fun(x + 1, y + 1);
        }
    }
}

impl Image<Rgb> {
    pub fn dump_ppm(&self, out: impl Write) -> io::Result<()> {
        let mut out = io::BufWriter::new(out);

        writeln!(
            out,
            r#"P6
{width} {height}
255"#,
            width = self.width,
            height = self.height
        )?;

        for (r, g, b) in &self.data {
            out.write_all(&[*r, *g, *b])?;
        }

        Ok(())
    }
}

fn color_dist((r1, g1, b1): Rgb, (r2, g2, b2): Rgb) -> i32 {
    let dr = i32::from(r1) - i32::from(r2);
    let dg = i32::from(g1) - i32::from(g2);
    let db = i32::from(b1) - i32::from(b2);

    dr * dr + dg * dg + db * db
}
