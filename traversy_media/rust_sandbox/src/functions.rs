pub fn run() {
    greeting("Hello", "World");

    // Bind function values ot variables
    let sum = add(1, 2);
    println!("1 + 2 = {}", sum);

    // Closure
    let n = 18;
    let add_nums = |x: i32, y: i32| x + y + n;
    println!("Closure sum: {}", add_nums(1, 2))
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}!", greet, name);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}