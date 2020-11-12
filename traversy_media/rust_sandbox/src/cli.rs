use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let first: String = args[1].clone();
    let second: String = if args.len() < 3 {
        "Brad".to_string()
    } else {
        args[2].clone()
    };

    println!("Args: {:?}", args);
    println!("First command: {}", first);

    if first == "hello" {
        println!("Hi {}, how are you?", second);
    } else if first == "status" {
        println!("Status is 100%");
    } else {
        println!("Wha?");
    }
}