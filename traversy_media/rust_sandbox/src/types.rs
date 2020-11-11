pub fn run() {
    // Default: i32
    let x = 1;
    // Default: f64
    let y = 2.5;
    // Add explicit type
    let z: i64 = 9223372036854775807;
    // Find max size
    println!("Max i32 = {}", std::i32::MAX);
    println!("Max i64 = {}", std::i64::MAX);
    // Boolean
    let is_active: bool = true;
    // Get boolean from expression
    let is_greater = 10 < 5;

    // char
    let a1 = 'a';
    let face = '\u{1f600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
