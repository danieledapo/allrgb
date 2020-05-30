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
