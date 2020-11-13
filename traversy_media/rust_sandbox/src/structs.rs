struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct
struct ColorTuple(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // construct Person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
    // get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    // set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }
    // name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut c1 = ColorTuple(255, 0, 0);
    c1.0 = 200;

    println!("ColorTuple: {} {} {}", c1.0, c1.1, c1.2);

    let mut p = Person::new("John", "Doe");

    println!("Person {} {}", p.first_name, p.last_name);
    println!("Person {}", p.full_name());

    p.set_last_name("Doez");

    println!("Person {}", p.full_name());
    println!("Person {:?}", p.to_tuple());
}
