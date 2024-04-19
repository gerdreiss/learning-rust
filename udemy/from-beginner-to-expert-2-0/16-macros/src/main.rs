macro_rules! our_macro {
    () => {
        1 + 1;
    };
    ($e1: expr, $e2: expr) => {
        $e1 + $e2
    };
}

macro_rules! input {
    ($t: ty) => {{
        let mut n = String::new();
        std::io::stdin()
            .read_line(&mut n)
            .expect("Failed to read input");

        let n: $t = n.trim().parse().expect("Invalid input");
        n
    }};
}

fn main() {
    println!("{}", our_macro!());
    println!("{}", our_macro!(10, 3));

    let some_input_0 = input!(f32);
}
