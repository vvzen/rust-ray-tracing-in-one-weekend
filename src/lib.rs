use std::fmt;
use std::ops;

pub type Point = Vec3;
pub type Color = Vec3;

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

    pub fn cross(self, other: Vec3) -> Vec3 {
        let new = Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.y * other.x,
            z: self.x * other.y - self.y * other.x,
        };

        new
    }
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
