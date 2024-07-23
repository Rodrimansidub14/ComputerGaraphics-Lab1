pub trait Line {
    fn draw_line(&mut self, x0: isize, y0: isize, x1: isize, y1: isize);
    fn draw_line_aa(&mut self, x0: isize, y0: isize, x1: isize, y1: isize);
}

impl Line for crate::framebuffer::Framebuffer {
    fn draw_line(&mut self, x0: isize, y0: isize, x1: isize, y1: isize) {
        let mut x0 = x0;
        let mut y0 = y0;
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            self.point(x0, y0);
            if x0 == x1 && y0 == y1 { break; }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    fn draw_line_aa(&mut self, x0: isize, y0: isize, x1: isize, y1: isize) {
        let mut x0 = x0 as f32;
        let mut y0 = y0 as f32;
        let x1 = x1 as f32;
        let y1 = y1 as f32;

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();

        let sx = if x0 < x1 { 1.0 } else { -1.0 };
        let sy = if y0 < y1 { 1.0 } else { -1.0 };

        let mut err = dx - dy;
        let mut e2;
        let ed = if dx + dy == 0.0 { 1.0 } else { (dx * dx + dy * dy).sqrt() };

        loop {
            self.set_foreground_color(0xFFFFFF);
            self.point(x0 as isize, y0 as isize);
            self.set_foreground_color(0x000000);
            if x0 == x1 && y0 == y1 { break; }
            e2 = err;
            let x2 = x0;
            if 2.0 * e2 >= -dx {
                if x0 == x1 { break; }
                if e2 + dy < ed { self.point(x0 as isize, y0 as isize); }
                err -= dy;
                x0 += sx;
            }
            if 2.0 * e2 <= dy {
                if y0 == y1 { break; }
                if dx - e2 < ed { self.point(x2 as isize, y0 as isize); }
                err += dx;
                y0 += sy;
            }
        }
    }
}
