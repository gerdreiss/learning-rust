#[derive(Debug)]
struct DrawingInfo {
    line_width: u8,
    color: String,
}

#[derive(Debug)]
struct Square {
    side: f32,
    drawing_info: DrawingInfo,
}

#[derive(Debug)]
struct Rectangle {
    length: f32,
    width: f32,
    drawing_info: DrawingInfo,
}

trait Draw {
    fn draw_object(&self);
}

trait Shape: Draw {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
}

impl Shape for Square {
    fn area(&self) -> f32 {
        self.side * self.side
    }
    fn perimeter(&self) -> f32 {
        4.0 * self.side
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.length * self.width
    }
    fn perimeter(&self) -> f32 {
        2.0 * (self.length + self.width)
    }
}

impl Draw for Square {
    fn draw_object(&self) {
        println!(
            "Drawing square with side: {}, line width: {}, color: {}",
            self.side, self.drawing_info.line_width, self.drawing_info.color
        );
    }
}

impl Draw for Rectangle {
    fn draw_object(&self) {
        println!(
            "Drawing rectangle with length: {}, width: {}, line width: {}, color: {}",
            self.length, self.width, self.drawing_info.line_width, self.drawing_info.color
        );
    }
}

fn shape_properties<T>(object: &T) -> (f32, f32)
where
    T: Shape,
{
    (object.area(), object.perimeter())
}

fn returns_shape(dim: Vec<f32>) -> Box<dyn Shape> {
    if dim.len() == 1 {
        let sq = Square {
            side: 5.0,
            drawing_info: DrawingInfo {
                line_width: 2,
                color: String::from("red"),
            },
        };
        Box::new(sq)
    } else {
        let rec = Rectangle {
            length: 10.0,
            width: 5.0,
            drawing_info: DrawingInfo {
                line_width: 2,
                color: String::from("blue"),
            },
        };
        Box::new(rec)
    }
}

fn main() {
    let r = Rectangle {
        length: 10.0,
        width: 5.0,
        drawing_info: DrawingInfo {
            line_width: 2,
            color: String::from("blue"),
        },
    };
    let s = Square {
        side: 5.0,
        drawing_info: DrawingInfo {
            line_width: 2,
            color: String::from("red"),
        },
    };

    println!("Area of rectangle: {}", r.area());
    println!("Perimeter of rectangle: {}", r.perimeter());
    println!("Area of square: {}", s.area());
    println!("Perimeter of square: {}", s.perimeter());
    println!(
        "Area and Perimeter of the rectangle: {:?}",
        shape_properties(&r)
    );
    println!(
        "Area and Perimeter of the square: {:?}",
        shape_properties(&s)
    );
    println!("Shapes: {:?}, {:?}", &s, &r)
}
