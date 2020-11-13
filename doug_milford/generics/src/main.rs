// struct Point {
//     x: i32,
//     y: i32,
// }
// struct Point {
//     x: f64,
//     y: f64,
// }

use std::fmt;
use std::ops;

struct Point<T, U> {
    x: T,
    y: U,
}

#[derive(Debug)]
enum E1<T> {
    OptionA(T),
    OptionB(T),
    OptionC,
}

trait SomeTrait {
    fn func(&self, a: &str, b: &str) -> String;
}

#[derive(Debug)]
struct SomeStruct {
    x: i32,
}

impl SomeTrait for SomeStruct {
    fn func(&self, a: &str, b: &str) -> String {
        self.x.to_string() + " - " + a + " - " + b
    }
}

impl SomeTrait for i32 {
    fn func(&self, a: &str, b: &str) -> String {
        "i32".to_string() + " - " + a + " - " + b
    }
}

#[allow(dead_code)]
struct SomeOtherTrait<T, U> {
    t: T,
    u: U,
}

impl<T, U> SomeOtherTrait<T, U>
where
    T: fmt::Debug,
    U: fmt::Debug,
{
    fn log_it(&self) {
        println!("{:?} {:?}", self.t, self.u);
    }
}

fn main() {
    let p = Point { x: 100, y: -1 };
    println!("x = {} y = {}", p.x, p.y);

    let b = Point { x: 10., y: 1. };
    println!("x = {} y = {}", b.x, b.y);

    let e1 = E1::OptionA(24.2);
    match e1 {
        E1::OptionA(a) => println!("OptionA contains {}", a),
        E1::OptionB(b) => println!("OtpionB contains {}", b),
        E1::OptionC => println!("OptionC"),
    };

    let e11 = E1::OptionB(vec![1, 2, 3]);
    println!("{:?}", e11);

    println!("a + b = {}", some_func(1, 10));

    let o = SomeStruct { x: 1000 };
    let r = o.func("a", "b");

    let a_i32 = 18;
    let r2 = do_this(&a_i32);
}

#[allow(dead_code)]
fn do_this<T>(v: &T) -> String
where
    T: SomeTrait + fmt::Debug,
{
    println!("{:?}", v);
    v.func("a", "b")
}

#[allow(dead_code)]
fn do_that(v: &dyn SomeTrait) -> String {
    //println!("{:?}", v); // does not compile cuz no Debug
    v.func("a", "b")
}

fn some_func<T: ops::Add<Output = T> + ops::Sub<Output = T> + fmt::Debug>(a: T, b: T) -> T {
    println!("a + b = {:?}", a);

    a - b
}

#[allow(dead_code)]
fn some_func2<T, E>(a: T, b: T, e: E) -> T
where
    T: ops::Add<Output = T> + ops::Sub<Output = T> + fmt::Debug,
    E: fmt::Debug,
{
    println!("a = {:?}", a);
    println!("e = {:?}", e);

    a - b
}
