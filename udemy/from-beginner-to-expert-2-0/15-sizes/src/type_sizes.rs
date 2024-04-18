use std::mem::size_of;

struct S {
    x: bool,
    y: i64,
}

impl S {
    fn new(x: bool, y: i64) -> Self {
        Self { x, y }
    }
}

pub fn print_type_sizes() {
    // sized types
    println!("i32 size: {}", size_of::<i32>());
    println!("i64 size: {}", size_of::<i64>());
    println!("(i32,i32) size: {}", size_of::<(i32, i32)>());
    println!("[i32; 3] size: {}", size_of::<[i32; 3]>());
    println!("&[i32] size: {}", size_of::<&[i32]>());
    println!("S size: {}", size_of::<S>());
    println!("Option<i32> size: {}", size_of::<Option<i32>>());
    println!("i32 reference: {}", size_of::<&i32>());
    println!("i32 mutable reference: {}", size_of::<&mut i32>());
    println!("Machine word size: {}", size_of::<&()>());
    println!("Box<i32> size: {}", size_of::<Box<i32>>());

    // Unsized types
    //println!("[i32] size: {}", size_of::<[i32]>()); cannot be known at compile time
    //println!("str size: {}", size_of::<str>()); cannot be known at compile time
}
