pub trait Line {
    fn draw_line(&mut self, x0: isize, y0: isize, x1: isize, y1: isize);
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
}
