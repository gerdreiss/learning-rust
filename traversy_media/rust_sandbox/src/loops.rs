pub fn run() {
    let mut count: i32 = 0;
    loop {
        count += 1;
        println!("Number {}", count);

        if count == 20 {
            break;
        }
    }

    for x in 0..40 {
        print!("=")
    }
    println!("\n");
    println!("\n");

    // while (FizzBuzz)
    while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", count);
        }
        // inc
        count += 1;
    }

    for x0 in 0..40 {
        print!("=")
    }
    println!("\n");
    println!("\n");

    // for range
    for x in 0..100 {
        if x % 15 == 0 {
            println!("fizzbuzz");
        } else if x % 3 == 0 {
            println!("fizz");
        } else if x % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", x);
        }
    }

    for x1 in 0..40 {
        print!("=")
    }
    println!("\n");
    println!("\n");
}