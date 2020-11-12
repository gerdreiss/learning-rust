#[derive(Clone, Copy)]
enum Movement {
    // variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    // perform action depending on  info
    match m {
        Movement::Left => println!("Avatar moving left"),
        Movement::Up => println!("Avatar moving up"),
        Movement::Right => println!("Avatar moving right"),
        Movement::Down => println!("Avatar moving down")
    }
}

pub fn run() {
    let avatar1 = Movement::Up;
    let avatar2 = Movement::Left;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;
    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
    move_avatar(avatar4);
}