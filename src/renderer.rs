use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

pub fn build_ppm_header(size: (u16, u16)) -> String {
    format!("P3 {} {} 255\n", size.0, size.1)
}

pub fn write_data_to_file(data: &[u8], writer: &mut BufWriter<File>) {
    for &c in data {
        writer
            .write_all(format!("{c} ").as_bytes())
            .expect("Couldn't write byte!");
    }
    writer.flush().expect("Failed to flush BufWriter!");
}

pub(crate) enum Format {
    Ppm,
    Png,
}

pub(crate) struct Renderer {
    size: (u16, u16),
    buffer: Vec<u8>,
}

impl Renderer {
    pub fn new(size: (u16, u16)) -> Renderer {
        Renderer {
            size,
            buffer: vec![0],
        }
    }
    pub fn render(&mut self) {
        self.buffer
            .resize((self.size.0 as u64 * self.size.1 as u64 * 3u64) as usize, 0);
        for x in 0..self.size.0 as u64 {
            for y in 0..self.size.1 as u64 {
                let r = ((x as f64 / self.size.0 as f64) * 255.0f64).round() as u8;
                let g = ((y as f64 / self.size.0 as f64) * 255.0f64).round() as u8;
                let b = ((0 as f64 / self.size.0 as f64) * 255.0f64).round() as u8;
                let idx = (x * self.size.0 as u64 + y) as usize * 3usize;
                self.buffer[idx] = r;
                self.buffer[idx + 1] = g;
                self.buffer[idx + 2] = b;
            }
        }
    }
    pub fn output(&self, format: Format, path: PathBuf) {
        // TODO: Check if data exists
        match format {
            Format::Ppm => {
                // Create the file
                let file = File::create(path).expect("Couldn't create file!");
                let mut writer = BufWriter::new(file);

                // Write PPM Header to file
                let header = build_ppm_header(self.size);
                writer
                    .write_all(header.as_bytes())
                    .expect("Couldn't write header to file!");

                // Write content to file
                write_data_to_file(&self.buffer, &mut writer);
            }
            Format::Png => {
                println!("WIP");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_build_ppm_header() {
        assert_eq!(
            build_ppm_header((52, 975)),
            String::from_str("P3 52 975 255").unwrap()
        );
    }
}
