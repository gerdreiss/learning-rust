use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("Mutable Vector: {:?}", numbers);
    println!("First value: {}", numbers[0]);

    numbers[2] = 20;
    println!("Mutable Vector: {:?}", numbers);
    println!("Changed value: {}", numbers[2]);
    println!("Vector length: {}", numbers.len());
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));


    numbers.push(6);
    println!("Mutable Vector: {:?}", numbers);

    numbers.pop();
    println!("Mutable Vector: {:?}", numbers);

    let slice: &[i32] = &numbers[1..3];
    println!("Vector slice: {:?}", slice);

    for x in numbers.iter() {
        println!("Looped num: {}", x);
    }

    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Modified Vector: {:?}", numbers);

}
