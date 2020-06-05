use std::io::{self, Write};

pub type Rgb = (u8, u8, u8);

pub struct Image<Pixel = Rgb> {
    data: Vec<Pixel>,
    width: usize,
    height: usize,
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

    #[inline(always)]
    pub fn get(&self, x: usize, y: usize) -> Pixel {
        if cfg!(debug_assertions) {
            self.data[y * self.width + x]
        } else {
            unsafe { *self.data.get_unchecked(y * self.width + x) }
        }
    }

    #[inline(always)]
    pub fn set(&mut self, x: usize, y: usize, pix: Pixel) {
        if cfg!(debug_assertions) {
            self.data[y * self.width + x] = pix;
        } else {
            unsafe {
                *self.data.get_unchecked_mut(y * self.width + x) = pix;
            }
        }
    }

    pub fn for_each_neighbor(&self, x: usize, y: usize, mut fun: impl FnMut(usize, usize)) {
        if x > 0 {
            fun(x - 1, y);
        }
        if y > 0 {
            fun(x, y - 1);
        }
        if y + 1 < self.height {
            fun(x, y + 1);
        }
        if x + 1 < self.width {
            fun(x + 1, y);
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
