struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Point<T, U> {
        Point { x, y }
    }
}

impl Point<i32, i32> {
    fn print(&self) {
        println!(
            "The values of the coordinates are x: {} and y: {}",
            self.x, self.y
        );
    }
}

impl Point<f64, f64> {
    fn print(&self) {
        println!(
            "The values of the coordinates are x: {} and y: {}",
            self.x, self.y
        );
    }
}

fn add_points<T, U>(p1: &Point<T, U>, p2: &Point<T, U>) -> Point<T, U> {
    unimplemented!();
}

fn main() {
    let origin: Point<i32, i32> = Point::new(0, 0);
    let p1: Point<f64, f64> = Point::new(1.0, 2.0);
    let p2: Point<i32, f64> = Point::new(1, 2.0);

    origin.print();
    p1.print();
    // p2.print();

    add_points(&p1, &p1);
    add_points(&p2, &p2);
}
