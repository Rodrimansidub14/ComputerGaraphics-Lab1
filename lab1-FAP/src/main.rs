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

<<<<<<< Updated upstream
    // Save the framebuffer as a BMP file
=======

    // Dibujar el primer polígono con borde blanco más grueso
    draw_polygon(&mut framebuffer, &polygon1, 0xFFFFFF, 2); // Orilla blanca con grosor 2

    // Rellenar el primer polígono con color amarillo
    fill_polygon(&mut framebuffer, &polygon1, 0xFFFF00); // Relleno amarillo

    let polygon2 = vec![
        (521, 235), (488, 186), (539, 151), (574, 202),
    ];

    // Dibujar el segundo polígono con borde blanco más grueso
    draw_polygon(&mut framebuffer, &polygon2, 0xFFFFFF, 2); // Orilla blanca con grosor 2

    // Rellenar el segundo polígono con color azul
    fill_polygon(&mut framebuffer, &polygon2, 0x0000FF); // Relleno azul

    // Guardar el framebuffer en un archivo BMP
>>>>>>> Stashed changes
    framebuffer.render_buffer("output.bmp").expect("Failed to save BMP file");

    println!("Framebuffer rendered to output.bmp");
}
