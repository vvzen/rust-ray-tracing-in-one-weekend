mod lib;

fn main() {
    let a = lib::Point {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    println!("{}", &a);

    let b = lib::Point {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let c = a + b;
    println!("{:?} + {:?} = {:?}", &a, &b, c);

    let d = a * 2.0;
    println!("{:?} * 2.0 = {:?}", &a, d);

    let e = 2.0 * a;
    println!("2.0 * {:?} = {:?}", &a, e);

    let f = a / 2.0;
    println!("{:?} / 2.0 = {:?}", &a, f);

    let g = a.length();
    println!("{:?}.length() = {:?}", &a, g);

    let h = -a;
    println!("-{:?} = {:?}", &a, h);

    let i = a.unit_vector();
    println!("unit vector of {:?}= {:?}", &a, i);

    let j = a.cross(b);
    println!("cross() of {:?} and {:?} = {:?})", &a, &b, j);
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
