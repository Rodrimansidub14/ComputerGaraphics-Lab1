pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    buffer: Vec<u32>,
    background_color: u32,
    foreground_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize, background_color: u32, foreground_color: u32) -> Self {
        Self {
            width,
            height,
            buffer: vec![background_color; width * height],
            background_color,
            foreground_color,
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
    }

    pub fn point(&mut self, x: isize, y: isize) {
        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            let index = (y as usize * self.width + x as usize) as usize;
            self.buffer[index] = self.foreground_color;
        }
    }

    pub fn get_color(&self, x: isize, y: isize) -> Option<u32> {
        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            let index = (y as usize * self.width + x as usize) as usize;
            Some(self.buffer[index])
        } else {
            None
        }
    }

    pub fn set_foreground_color(&mut self, color: u32) {
        self.foreground_color = color;
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
        self.clear();
    }

    pub fn render_buffer(&self, file_path: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;

        let file_header_size = 14;
        let info_header_size = 40;
        let pixel_data_offset = file_header_size + info_header_size;
        let file_size = pixel_data_offset + (self.buffer.len() * 3) as u32;

        let mut file = File::create(file_path)?;

        file.write_all(&[
            0x42, 0x4D, // Signature "BM"
            (file_size & 0xFF) as u8, ((file_size >> 8) & 0xFF) as u8,
            ((file_size >> 16) & 0xFF) as u8, ((file_size >> 24) & 0xFF) as u8,
            0, 0, 0, 0, 
            (pixel_data_offset & 0xFF) as u8, ((pixel_data_offset >> 8) & 0xFF) as u8,
            ((pixel_data_offset >> 16) & 0xFF) as u8, ((pixel_data_offset >> 24) & 0xFF) as u8,
        ])?;

        // BMP info header
        file.write_all(&[
            (info_header_size & 0xFF) as u8, ((info_header_size >> 8) & 0xFF) as u8,
            ((info_header_size >> 16) & 0xFF) as u8, ((info_header_size >> 24) & 0xFF) as u8,
            (self.width & 0xFF) as u8, ((self.width >> 8) & 0xFF) as u8,
            ((self.width >> 16) & 0xFF) as u8, ((self.width >> 24) & 0xFF) as u8,
            (self.height & 0xFF) as u8, ((self.height >> 8) & 0xFF) as u8,
            ((self.height >> 16) & 0xFF) as u8, ((self.height >> 24) & 0xFF) as u8,
            1, 0, // Planes
            24, 0, // Bits per pixel
            0, 0, 0, 0, // Compression
            0, 0, 0, 0, // Image size (can be 0 for uncompressed images)
            0x13, 0x0B, 0, 0, // X pixels per meter (2835)
            0x13, 0x0B, 0, 0, // Y pixels per meter (2835)
            0, 0, 0, 0, // Total colors
            0, 0, 0, 0, // Important colors
        ])?;

        // BMP pixel data (bottom to top)
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let color = self.buffer[y * self.width + x];
                file.write_all(&[
                    (color & 0xFF) as u8,        // Blue
                    ((color >> 8) & 0xFF) as u8, // Green
                    ((color >> 16) & 0xFF) as u8, // Red
                ])?;
            }
        }

        Ok(())
    }
}
