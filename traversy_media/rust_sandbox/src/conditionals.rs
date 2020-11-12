pub fn run() {
    let age: i32 = 30;
    let check_id: bool = true;
    let knows_person: bool = true;

    // if else
    if age >= 21 && check_id || knows_person {
        println!("Bartender: What's gonna be?")
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, get out!")
    } else {
        println!("Bartender: got an id?")
    }

    // shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);

}