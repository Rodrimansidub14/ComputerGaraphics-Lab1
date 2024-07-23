mod framebuffer;
mod line_impl;
mod bmp;

use framebuffer::Framebuffer;
use line_impl::Line;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height, 0xFFFFFF, 0x000000);

    // Clear the framebuffer with a white background
    framebuffer.set_background_color(0xFFFFFF);
    framebuffer.clear();

    // Set the current drawing color to black
    framebuffer.set_foreground_color(0x000000);

    // Example line
    framebuffer.draw_line(100, 100, 700, 500);
    framebuffer.draw_line(700, 100, 100, 500);

    // Save the framebuffer as a BMP file
    framebuffer.render_buffer("output.bmp").expect("Failed to save BMP file");

    println!("Framebuffer rendered to output.bmp");
}
