#[derive(Debug, Clone, Copy)]
struct SomeStruct {
    x: i32,
    y: f64,
    // s: String, <- this would break the Copy derivation
}
// Implement Clone for SomeStruct
// impl Clone for SomeStruct {
//     fn clone(&self) -> Self {
//         Self { x: self.x, y: self.y }
//     }
// }

fn main() {
    stack_vs_heap();
    stack_vs_heap_structs();
}

#[allow(unused_variables)] // this is so we don't get warnings throughout the exercise
fn stack_vs_heap() {
    let var_a = String::from("Howdy");
    let var_b = var_a;
    //println!("{}", var_a); // this does not compile

    // STACK
    // Fast memory creation and retrieval... speed, speed, SPEED!
    // Memory is automatically recaptured by the program after variables go out of scope
    // Is the default in Rust
    // Fixed in size variables
    // Collections CANNOT be stack because variable in size
    // Strings are a collection of u8's
    let stack_i8_1: i8 = 10;
    let stack_f32: f32 = 20.;
    let stack_bool: bool = true;
    let stack_char: char = 'a';

    // create new scope with a variable that is not visible outside of that scope
    if stack_i8_1 == 3 {
        let inside_scope = 9;
        println!("{}", inside_scope);
    }
    println!();

    // HEAP
    // Flexibility
    // memory that can grow in size (Vector, HashMap, String, etc)
    // Runtime performance cost (speed)
    // Memory that can live beyond the scope that created it
    // Memory is automatically recaptured when the last OWNER goes out of scope
    let heap_vector: Vec<i8> = Vec::new();
    let heap_string: String = String::from("Howdy");

    // this works fine cuz that stuff is on the stack
    let stack_i8_2 = stack_i8_1;

    println!("Original: {}", stack_i8_1);
    println!("Copied:   {}", stack_i8_2);
    println!();

    // ================================================================================
    // this doesn't work cuz of ownership, unless I clone or borrow that memory
    let heap_i8_1: Box<i8> = Box::new(30);
    let heap_i8_2: Box<i8> = heap_i8_1.clone(); // <- do this not to lose the original i8 (expensive)
    let heap_i8_3: &Box<i8> = &heap_i8_1; // or this (borrow, cheaper)

    println!("Original: {}", heap_i8_1);
    println!("Cloned:   {}", heap_i8_2);
    println!("Borrowed: {}", heap_i8_3);
    println!();
    println!();

    let stack_f64: f64 = 1.;
    let heap_f64: Box<f64> = Box::new(2.);

    stack_procedure(stack_f64);
    println!("In main stack {}", stack_f64);

    heap_procedure(&heap_f64); // borrow that sh*t, do not clone -> expensive af!
    println!("In main heap {}", heap_f64);


    // ================================================================================
    let some_string: String = String::from("Howdy"); // heap
    let some_str: &str = "Partner"; // &str is a pointer to either stack or heap
    some_procedure(&some_string, some_str);
    println!("some_string: {}, some_str: {}", some_string, some_str);


    // ================================================================================
    let mut a = String::from("Howdy!");

    a.push('!');

    let b = &a;
    let c = &a;

    //a.push('!'); // DOES NOT COMPILE CUZ b AND c rely on the string being immutable

    println!("a: {}, b: {}, c: {}", a, b, c);

    a.push('!'); // this works cause a is being changed after b and c have been used

    // println!("{}", c); this wouldn't work cuz a has been changed above
}

fn stack_procedure(mut param: f64) {
    param += 9.;
    println!("In stack_procedure with param {}", param);
}

fn heap_procedure(param: &Box<f64>) {
    println!("In heap_procedure with param {}", param);
}

fn some_procedure(some_string: &String, some_str: &str) {
    println!("some_string: {}, some_str: {}", some_string, some_str);
}

fn stack_vs_heap_structs() {
    let ss = SomeStruct { x: 10, y: 10. };
    some_struct_proc(ss); // to be able to do this derive or impl Clone AND Copy!
    println!("{:?}", ss);
}

fn some_struct_proc(ss: SomeStruct) {
    println!("{:?}", ss);
}