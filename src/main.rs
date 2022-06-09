mod lib;

fn main() {
    lib::write_sample_bg();
    //lib::write_sample_image();
}

#[cfg(test)]
mod tests {

    use crate::lib;

    #[test]
    fn test_vec_addition() {
        let a = lib::Point {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let b = lib::Point {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let result = lib::Point {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };
        assert_eq!(a + b, result);
    }

    #[test]
    fn test_f32_sub() {
        let a = lib::Point {
            x: 6.0,
            y: 6.0,
            z: 6.0,
        };
        let b = 3.0;
        let result = lib::Point {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };
        assert_eq!(a / b, result);
    }
}
