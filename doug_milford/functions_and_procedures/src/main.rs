use rand;
use rand::Rng;

#[allow(unused_variables)]
fn main() {
    let mut rng = rand::thread_rng();

    let returned_data = some_function(rng.gen::<f32>(), rng.gen::<i128>());
    println!("returned data: {}", returned_data);

    some_procedure(rng.gen::<f32>(), rng.gen::<i64>());
    some_str_procedure("boom!");

    let string_slice_var: &str = "Howdy!";
    some_str_procedure(string_slice_var);

    let string_var: String = String::from("Howey!!");
    some_str_procedure(&string_var);
    some_str_procedure(&string_var);
    some_str_procedure(&string_var);
    some_string_procedure(string_var);
    //some_string_procedure(string_var); does not compile -> borrowed already above
}

fn some_function(x: f32, y: i128) -> f32 {
    println!("I'm in some_function with params {} and {}", x, y);
    10. * x + y as f32 // don't do this
}

// procedure because no return type
fn some_procedure(x: f32, y: i64) {
    println!("I'm in some_procedure with params {} and {}", x, y);
}

fn some_str_procedure(s: &str) {
    println!("I'm in some_str_procedure with param '{}'", s);
}

fn some_string_procedure(s: String) {
    println!("I'm in some_string_procedure with param '{}'", s);
}