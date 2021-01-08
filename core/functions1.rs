///
/// A function implementation for impl for struct
///
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn origin() -> Point {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn new_point(x:f64, y:f64, z:f64) -> Point {
        Point {
            x: x,
            y: y,
            z: z,
        }
    }
}

#[derive(Debug)]
struct MakePoint {
    x:Point,
    y: Point,
    z: Point,
}

fn main() {
    let universe = MakePoint {
        x: Point::origin(),
        y: Point::new_point(2.3,23.3,2.3),
        z: Point::new_point(34.4,34.0,2.3),
    };
    println!("{:?}", universe)
}
