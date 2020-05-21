#[derive(Debug)]
struct Point(f64, f64);

#[derive(Debug)]
struct Circle {
    center: Point,
    radius: f64,
}

impl Default for Point {
    fn default() -> Point {
        Point(1.0, 1.0)
    }
}

impl Default for Circle {
    fn default() -> Circle {
        Circle {
            center: Point::default(),
            radius: 1.0,
        }
    }
}

fn make_default<T: Default>() -> T {
    T::default()
}

fn make_default_pair<T>() -> (T, T)
where T:Default {
    (make_default(), make_default())
}

fn main() {
    let p: Point = make_default();
    println!("{:?}", p);

    let (c1, c2): (Circle, Circle) = make_default_pair();
    println!("{:?} {:?}", c1, c2);
}
