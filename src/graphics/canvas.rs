use std::{
    fmt,
    fs::File,
    io::{stdout, Write},
};

use crate::math::Tuple;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub matrix: Vec<Vec<Tuple>>,
}

#[allow(dead_code)]
impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let matrix = vec![vec![Tuple::color(0.0, 0.0, 0.0); width]; height];
        Self {
            width,
            height,
            matrix,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, c: &Tuple) {
        self.matrix[y][x] = *c;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> &Tuple {
        &self.matrix[y][x]
    }

    pub fn write_to_file(&self, path: &str) -> std::io::Result<()> {
        print!("Writing to file... ");
        stdout().flush()?;
        let mut file = File::create(path)?;
        write!(file, "{}", self)?;
        println!("Done");
        Ok(())
    }
}

impl fmt::Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "P3")?;
        writeln!(f, "{} {}", self.width, self.height)?;
        writeln!(f, "255")?;
        for row in &self.matrix {
            for (i, p) in row.iter().enumerate() {
                let r = (255.0 * p.red().clamp(0.0, 1.0)).clamp(0.0, 255.0).round() as u8;
                let g = (255.0 * p.green().clamp(0.0, 1.0))
                    .clamp(0.0, 255.0)
                    .round() as u8;
                let b = (255.0 * p.blue().clamp(0.0, 1.0)).clamp(0.0, 255.0).round() as u8;
                write!(f, "{} {} {}", r, g, b)?;
                if i < self.width - 1 {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::math::Tuple;

    use super::Canvas;

    #[test]
    fn canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
    }

    #[test]
    fn writing_pixel() {
        let mut c = Canvas::new(10, 20);
        let r = Tuple::color(1.0, 0.0, 0.0);
        c.set_pixel(2, 3, &r);
        assert_eq!(c.get_pixel(2, 3), &r);
    }

    #[test]
    fn to_canvas() {
        let c = Canvas::new(5, 3);
        let output = format!("{}", c);
        let lines: Vec<&str> = output.split('\n').collect();
        assert_eq!(lines[0], "P3");
        assert_eq!(lines[1], "5 3");
        assert_eq!(lines[2], "255");
    }

    #[test]
    fn to_big_canvas() {
        let mut c = Canvas::new(5, 3);
        c.set_pixel(0, 0, &Tuple::color(1.5, 0.0, 0.0));
        c.set_pixel(2, 1, &Tuple::color(0.0, 0.5, 0.0));
        c.set_pixel(4, 2, &Tuple::color(-0.5, 0.0, 1.0));
        let output = format!("{}", c);
        let lines: Vec<&str> = output.split('\n').collect();
        assert_eq!(lines[0], "P3");
        assert_eq!(lines[1], "5 3");
        assert_eq!(lines[2], "255");
        assert_eq!(lines[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(lines[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
        assert_eq!(lines[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }

    #[test]
    fn ends_with_newline() {
        let c = Canvas::new(5, 3);
        let output: String = format!("{}", c);
        assert_eq!(output.chars().last(), Some('\n'));
    }
}
