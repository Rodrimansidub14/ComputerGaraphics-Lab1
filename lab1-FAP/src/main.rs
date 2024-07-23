mod framebuffer;
mod line_impl;
mod bmp;
mod polygon;

use framebuffer::Framebuffer;
use polygon::{draw_polygon, fill_polygon};

fn main() {
    let width = 800;
    let height = 600;
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
    fill_polygon(&mut framebuffer, &polygon1, 0xFFFF00); // Relleno amarillo

    // Definir los puntos del segundo polígono centrado en el cuadrante superior derecho
    // Ajustando las coordenadas para moverlo un poco a la izquierda y hacia abajo
    let polygon2 = vec![
        (521, 135), (488, 86), (539, 51), (574, 102),
    ];

    // Dibujar el segundo polígono con borde blanco más grueso
    draw_polygon(&mut framebuffer, &polygon2, 0xFFFFFF, 2); // Orilla blanca con grosor 2

    // Rellenar el segundo polígono con color azul
    fill_polygon(&mut framebuffer, &polygon2, 0x0000FF); // Relleno azul

    // Definir los puntos del tercer polígono centrado en la parte inferior
    let polygon3 = vec![
        (377, 500), (411, 450), (436, 500),
    ];

    // Dibujar el tercer polígono con borde blanco más grueso
    draw_polygon(&mut framebuffer, &polygon3, 0xFFFFFF, 2); // Orilla blanca con grosor 2

    // Rellenar el tercer polígono con color rojo
    fill_polygon(&mut framebuffer, &polygon3, 0xFF0000); // Relleno rojo

    // Guardar el framebuffer en un archivo BMP
    framebuffer.render_buffer("output.bmp").expect("Failed to save BMP file");

    println!("Framebuffer rendered to output.bmp");
}
