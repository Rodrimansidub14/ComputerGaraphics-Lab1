use crate::framebuffer::Framebuffer;
use crate::line_impl::Line;

pub fn draw_polygon(framebuffer: &mut Framebuffer, points: &[(isize, isize)], border_color: u32, thickness: isize) {
    framebuffer.set_foreground_color(border_color);

    // Desplazamiento para aumentar el grosor de la línea
    for offset in -thickness..=thickness {
        for i in 0..points.len() {
            let (x0, y0) = points[i];
            let (x1, y1) = points[(i + 1) % points.len()];
            framebuffer.draw_line(x0 + offset, y0, x1 + offset, y1);
            framebuffer.draw_line(x0, y0 + offset, x1, y1 + offset);
        }
    }
}

pub fn fill_polygon(framebuffer: &mut Framebuffer, outer_points: &[(isize, isize)], holes: &[&[(isize, isize)]], fill_color: u32) {
    framebuffer.set_foreground_color(fill_color);
    let min_y = outer_points.iter().map(|&(_, y)| y).min().unwrap_or(0);
    let max_y = outer_points.iter().map(|&(_, y)| y).max().unwrap_or(0);

    for y in min_y..=max_y {
        let mut intersections = vec![];
        for i in 0..outer_points.len() {
            let (x0, y0) = outer_points[i];
            let (x1, y1) = outer_points[(i + 1) % outer_points.len()];

            if (y0 <= y && y < y1) || (y1 <= y && y < y0) {
                let x = x0 + (x1 - x0) * (y - y0) / (y1 - y0);
                intersections.push(x);
            }
        }
        for hole in holes {
            for i in 0..hole.len() {
                let (x0, y0) = hole[i];
                let (x1, y1) = hole[(i + 1) % hole.len()];

                if (y0 <= y && y < y1) || (y1 <= y && y < y0) {
                    let x = x0 + (x1 - x0) * (y - y0) / (y1 - y0);
                    intersections.push(x);
                }
            }
        }
        intersections.sort();
        for pair in intersections.chunks(2) {
            if let [x_start, x_end] = pair {
                for x in *x_start..=*x_end {
                    framebuffer.point(x, y);
                }
            }
        }
    }
}
