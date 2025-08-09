use std::ops::Deref;
use macroquad::miniquad::native::apple::frameworks::MTLPrimitiveType::Triangle;
use macroquad::miniquad::window::screen_size;
use macroquad::prelude::*;

#[macroquad::main("HB_RUST")]
async fn main() {
    let size = screen_size();
    print!("x: {}, y: {}", size.0.to_string(), size.1.to_string());
    next_frame().await;

    loop {

        clear_background(BLACK);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);
        draw_triangle();
        next_frame().await
    }
}