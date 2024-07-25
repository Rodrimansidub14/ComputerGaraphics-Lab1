mod framebuffer;
mod line_impl;
mod bmp;
mod polygon;

use framebuffer::Framebuffer;
use polygon::{draw_polygon, fill_polygon};

fn main() {
    let width = 800;
    let height = 600;
    let center_x = width as isize / 2;
    let center_y = height as isize / 2;
    let offset_y = 200; // Ajuste hacia abajo
    let mut framebuffer = Framebuffer::new(width, height, 0x000000, 0xFFFFFF); // Fondo negro y color inicial blanco

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    // Definir los puntos del primer polígono centrado en el cuadrante superior izquierdo
    let polygon1 = vec![
        (165, 180), (185, 160), (180, 130), (207, 145), (233, 130),
        (230, 160), (250, 180), (220, 185), (205, 210), (193, 183),
    ];

    // Dibujar el primer polígono con borde blanco más grueso
    draw_polygon(&mut framebuffer, &polygon1, 0xFFFFFF, 2); // Orilla blanca con grosor 2

    // Rellenar el primer polígono con color amarillo
    fill_polygon(&mut framebuffer, &polygon1, &[], 0xFFFF00); // Relleno amarillo

    // Definir los puntos del segundo polígono centrado en el cuadrante superior derecho
    let polygon2 = vec![
        (521, 135), (488, 86), (539, 51), (574, 102),
    ];

    // Dibujar el segundo polígono con borde blanco más grueso
    draw_polygon(&mut framebuffer, &polygon2, 0xFFFFFF, 2); // Orilla blanca con grosor 2

    // Rellenar el segundo polígono con color azul
    fill_polygon(&mut framebuffer, &polygon2, &[], 0x0000FF); // Relleno azul

    // Definir los puntos del tercer polígono centrado en la parte inferior
    let polygon3 = vec![
        (377, 500), (411, 450), (436, 500),
    ];

    // Dibujar el tercer polígono con borde blanco más grueso
    draw_polygon(&mut framebuffer, &polygon3, 0xFFFFFF, 2); // Orilla blanca con grosor 2

    // Rellenar el tercer polígono con color rojo
    fill_polygon(&mut framebuffer, &polygon3, &[], 0xFF0000); // Relleno rojo

    // Calcular el desplazamiento necesario para centrar el polígono 4 y el agujero (polígono 5)
    let polygon4 = vec![
        (413 - 400 + center_x, 177 - 300 + center_y + offset_y), (448 - 400 + center_x, 159 - 300 + center_y + offset_y),
        (502 - 400 + center_x, 88 - 300 + center_y + offset_y), (553 - 400 + center_x, 53 - 300 + center_y + offset_y),
        (535 - 400 + center_x, 36 - 300 + center_y + offset_y), (676 - 400 + center_x, 37 - 300 + center_y + offset_y),
        (660 - 400 + center_x, 52 - 300 + center_y + offset_y), (750 - 400 + center_x, 145 - 300 + center_y + offset_y),
        (761 - 400 + center_x, 179 - 300 + center_y + offset_y), (672 - 400 + center_x, 192 - 300 + center_y + offset_y),
        (659 - 400 + center_x, 214 - 300 + center_y + offset_y), (615 - 400 + center_x, 214 - 300 + center_y + offset_y),
        (632 - 400 + center_x, 230 - 300 + center_y + offset_y), (580 - 400 + center_x, 230 - 300 + center_y + offset_y),
        (597 - 400 + center_x, 215 - 300 + center_y + offset_y), (552 - 400 + center_x, 214 - 300 + center_y + offset_y),
        (517 - 400 + center_x, 144 - 300 + center_y + offset_y), (466 - 400 + center_x, 180 - 300 + center_y + offset_y),
    ];

    let polygon5 = vec![
        (682 - 400 + center_x, 175 - 300 + center_y + offset_y), (708 - 400 + center_x, 120 - 300 + center_y + offset_y),
        (735 - 400 + center_x, 148 - 300 + center_y + offset_y), (739 - 400 + center_x, 170 - 300 + center_y + offset_y),
    ];

    // Dibujar el cuarto polígono con borde blanco más grueso
    draw_polygon(&mut framebuffer, &polygon4, 0xFFFFFF, 2); // Orilla blanca con grosor 2

    // Rellenar el cuarto polígono con color verde y con el quinto polígono como agujero
    fill_polygon(&mut framebuffer, &polygon4, &[&polygon5], 0x00FF00); // Relleno verde

    // Guardar el framebuffer en un archivo BMP
    framebuffer.render_buffer("output.bmp").expect("Failed to save BMP file");

    println!("Framebuffer rendered to output.bmp");
}
