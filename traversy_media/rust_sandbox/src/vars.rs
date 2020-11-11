pub fn run() {
    let name = "Brad";
    let mut age = 47;
    println!("My name is {} and I am {}", name, age);
    age = 48;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assing multiple vars
    let (my_name, my_age) = ("Brad", "47");
    println!("{} is {}", my_name, my_age);
}
