fn main() {
    println!("Enter you weight on Earth (kg): ");
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Err(err) => println!("Error occurred while reading user input: {}", err),
        Ok(size) => {
            if size == 0 {
                println!("Nothing entered");
            } else {
                match parse_input(input) {
                    Err(err) => println!("Invalid input ({})", err),
                    Ok(weight) => {
                        println!("Weight on Mars: {}kg", calculate_weight_on_mars(weight))
                    }
                }
            }
        }
    }
}

fn parse_input(input: String) -> Result<f32, std::num::ParseFloatError> {
    input.trim().parse()
}

fn calculate_weight_on_mars(weight_on_earth: f32) -> f32 {
    weight_on_earth / 9.81 * 3.711
}
