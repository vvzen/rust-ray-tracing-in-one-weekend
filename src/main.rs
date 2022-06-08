mod lib;

fn main() {
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
    let c = a + b;

    println!("{:?} + {:?} = {:?}", &a, &b, c);
}
