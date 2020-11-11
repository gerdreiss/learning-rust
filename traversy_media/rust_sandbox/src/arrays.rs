use std::mem;

pub fn run() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", numbers);
    println!("First value: {}", numbers[0]);
    let mut mut_numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Mutable Array: {:?}", mut_numbers);
    println!("First value: {}", mut_numbers[0]);
    mut_numbers[2] = 20;
    println!("Mutable Array: {:?}", mut_numbers);
    println!("Changed value: {}", mut_numbers[2]);
    println!("Array length: {}", numbers.len());
    println!("Array occupies {} bytes", mem::size_of_val(&mut_numbers));

    let slice: &[i32] = &numbers[1..3];
    println!("Array slice: {:?}", slice);
}
