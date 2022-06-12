use std::fmt;
use std::ops;

pub type Point = Vec3;
pub type Color = Vec3;

pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    fn at(&self, t: f32) -> Point {
        let result = self.origin + t * self.direction;
        result
    }
}

pub fn ray_color(ray: &Ray) -> Color {
    let sphere_center = Point {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };

    // If we hit the sphere, draw the sphere
    let t = hit_sphere(sphere_center, 0.5, &ray);
    if t > 0.0 {
        let normal = (ray.at(t) - sphere_center).unit_vector();

        let color = Color {
            x: normal.x + 1.0,
            y: normal.y + 1.0,
            z: normal.z + 1.0,
        } * 0.5;
        return color;
    }

    // Draw the background
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    let white = Color {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    let bg_color = Color {
        x: 0.5,
        y: 0.7,
        z: 1.0,
    };

    // Linear interpolate to make a gradient background
    let result = (1.0 - t) * white + t * bg_color;

    result
}

#[derive(Debug, Copy, PartialEq, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn unit_vector(self) -> Vec3 {
        let new = self / self.length();

        new
    }
}

pub fn dot(a: Vec3, b: Vec3) -> f32 {
    let result = a.x * b.x + a.y * b.y + a.z * b.z;

    result
}

pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    let new = Vec3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.y * b.x,
        z: a.x * b.y - a.y * b.x,
    };

    new
}
// For more help on base traits that let us do 'operators overloading', see:
// https://doc.rust-lang.org/rust-by-example/trait/ops.html

// Here, we implement `Add<Vec3>` - the trait for addition with a RHS of type `Vec3`.
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        let new = Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };

        new
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        let new = Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };

        new
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        let new = Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        };

        new
    }
}

// LHS is a Vec3, RHS is a f32
impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        let new = Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        };
        new
    }
}

// LHS is a Vec3, RHS is a Vec3
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        let new = Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        };
        new
    }
}

// LHS is a f32, RHS is a Vec3
impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        let new = Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        };
        new
    }
}

// LHS is a i32, RHS is a Vec3
impl ops::Mul<Vec3> for i32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        let new = Vec3 {
            x: self as f32 * other.x,
            y: self as f32 * other.y,
            z: self as f32 * other.z,
        };
        new
    }
}

// LHS is a Vec3, RHS is a f32
impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        let new = self * (1.0 / other);
        new
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec3 {{ x: {}, y: {}, z: {} }}", self.x, self.y, self.z)
    }
}

pub fn print_color(color: &Vec3) {
    print!(
        "{} {} {}\n",
        (color.x * 255.999).floor() as i32,
        (color.y * 255.999).floor() as i32,
        (color.z * 255.999).floor() as i32
    );
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

            let color = Color { x: r, y: g, z: b };

            print_color(&color);
        }
    }
}

pub fn write_sample_bg() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400.0;
    let image_height = image_width / aspect_ratio;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let horizontal = Vec3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vec3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };
    let lower_left_corner = origin
        - horizontal / 2.0
        - vertical / 2.0
        - Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        };

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for y in (0..image_height as i32).rev() {
        eprintln!("\rScanlines remaining: {}", y);
        for x in 0..image_width as i32 {
            let u = x as f32 / (image_width - 1.0);
            let v = y as f32 / (image_height - 1.0);

            let direction = lower_left_corner + u * horizontal + v * vertical - origin;
            let ray = Ray { origin, direction };

            let color = ray_color(&ray);

            print_color(&color);
        }
    }
}

pub fn hit_sphere(center: Point, radius: f32, ray: &Ray) -> f32 {
    let oc: Point = ray.origin - center;

    let a = dot(ray.direction, ray.direction);
    let b = 2.0 * dot(oc, ray.direction);
    let c = dot(oc, oc) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
}
