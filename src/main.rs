use minifb::{Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() {
    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    // Fill the buffer with a gradient
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let r = (x as f32 / WIDTH as f32 * 255.0) as u32;
            let g = (y as f32 / HEIGHT as f32 * 255.0) as u32;
            let b = 0; // Fixed blue value
            buffer[y * WIDTH + x] = (r << 16) | (g << 8) | b; // ARGB format
        }
    }

    let mut window =
        Window::new("Rust Rasterizer", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
