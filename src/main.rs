use minifb::{Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

struct Vec2 {
    x: f32,
    y: f32,
}

struct Triangle3 {
    p0: Vec3,
    p1: Vec3,
    p2: Vec3,
}

impl Triangle3 {
    fn new(p0: Vec3, p1: Vec3, p2: Vec3) -> Self {
        Triangle3 { p0, p1, p2 }
    }

    pub fn in_triangle(&self, p: Vec3) -> bool {
        let area =
            |a: &Vec3, b: &Vec3, c: &Vec3| (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);

        let area1 = area(&p, &self.p1, &self.p2);
        let area2 = area(&self.p0, &p, &self.p2);
        let area3 = area(&self.p0, &self.p1, &p);

        (area1 >= 0.0 && area2 >= 0.0 && area3 >= 0.0)
            || (area1 <= 0.0 && area2 <= 0.0 && area3 <= 0.0)
    }
}

fn main() {
    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    // Create a triangle
    let triangle = Triangle3::new(
        Vec3 {
            x: 100.0,
            y: 100.0,
            z: 0.0,
        },
        Vec3 {
            x: 200.0,
            y: 50.0,
            z: 0.0,
        },
        Vec3 {
            x: 150.0,
            y: 200.0,
            z: 0.0,
        },
    );

    // Rasterize the triangle
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let p = Vec3 {
                x: x as f32,
                y: y as f32,
                z: 0 as f32,
            };
            if triangle.in_triangle(p) {
                buffer[y * WIDTH + x] = 0x00FF00; // Green color
            } else {
                buffer[y * WIDTH + x] = 0x000000; // Black color
            }
        }
    }

    let mut window = Window::new("Rusterize!", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
