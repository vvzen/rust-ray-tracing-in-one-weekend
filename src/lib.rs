use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// See: https://doc.rust-lang.org/rust-by-example/trait/ops.html
// The `std::ops::Add` trait is used to specify the functionality of `+`.
// Here, we make `Add<Point>` - the trait for addition with a RHS of type `Point`.
impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(mut self, other: Point) -> Point {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
        self
    }
}

pub fn write_sample_image() {
    let image_width = 256;
    let image_height = 256;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for y in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", y);
        for x in 0..image_width {
            let r = x as f32 / (image_width as f32 - 1.0);
            let g = y as f32 / (image_height as f32 - 1.0);
            let b = 0.25 as f32;

            print!(
                "{} {} {}\n",
                (r * 255.999).floor() as i32,
                (g * 255.999).floor() as i32,
                (b * 255.999).floor() as i32
            );
        }
    }
}
