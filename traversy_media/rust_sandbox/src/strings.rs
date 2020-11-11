pub fn run() {
    let mut hello = String::from("Hello ");
    hello.push('W');
    hello.push_str("orld!");

    let mut with_cap = String::with_capacity(10);
    with_cap.push('a');
    with_cap.push('b');
    assert_eq!(2, with_cap.len());
    assert_eq!(10, with_cap.capacity());

    println!("{}", hello);
    println!("Length: {}", hello.len());
    println!("Capacity: {}", hello.capacity());
    println!("Is empty: {}", hello.is_empty());
    println!("Contains 'World': {}", hello.contains("World"));
    println!("Replace: {}", hello.replace("World", "There"));

    // split by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }
}
