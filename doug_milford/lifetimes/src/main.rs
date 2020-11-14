const A: &str = "a constant";
const B: &str = "another constant";


struct Simple<'a, 'b: 'a> {
    aa: Vec<i32>,
    bb: &'a Vec<i32>,
    cc: &'b Vec<i32>
}

fn main() {
    let a;

    {
        let b = String::from("Howdy");
        a = b;
        // a = &b; <-- this does not compile
    }

    println!("{}", a);
    println!("{}", get_int_ref(&1, &2));


    let x = 1.1;
    let y = 2.1;
    let res0 = get_smaller(&x, &y);
    println!("{}", res0);

    let a = 'a';
    let b = 'b';
    let res1 = get_smaller(&a, &b);
    println!("{}", res1);


}

// defining a generic lifetime <'a>
// fn get_int_ref<'a, 'b, 'c>(x: &'a i32, y: &'b i32) -> &'c i32 {
//     let z = x + y;
//     &z
// }

fn get_int_ref<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
        x
    } else {
        y
    }
}

fn f(a: &'static str, b: &'static str) -> &'static str {
    if a > b {
        a
    } else {
        b
    }
}

fn get_smaller<'a, T: std::cmp::PartialOrd>(x: &'a T, y: &'a T) -> &'a T {
    if x < y {
        x
    } else {
        y
    }
}