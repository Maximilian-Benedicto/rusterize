use minifb::{Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() {
    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    // Draw a red pixel in the center
    let x = WIDTH / 2;
    let y = HEIGHT / 2;
    buffer[y * WIDTH + x] = 0x00FF0000;

    let mut window = Window::new("Rust Rasterizer", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
