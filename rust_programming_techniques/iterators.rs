use std::collections::HashMap;

struct Foo {
    f: i32,
}

impl Foo {
    fn ping(&self) {
        println!("Foo::ping({})", self.f);
    }
}

fn ping_all(foos: &[Foo]) {
    for f in foos {
        f.ping();
    }
}

fn main() {
    ping_all(&[Foo{f: 1}, Foo{f: 2}]);

    let vec = vec![1, 2, 3, 4, 5];

    vec.iter()
       .map( |x| x + 1)
       .filter( |x| x > &2)
       .for_each( |x| println!("{}", x));

    // enumerate is like zipWithIndex
    for (i, v) in vec.iter().chain(Some(42).iter()).enumerate() {
        println!("{}: {}", i, v);
    }    

    // for collect() to work here, the result type needs to be defined at the variable level
    let _vec: Vec<_> = vec.iter().map(|x| x * 2).collect();
    let map: HashMap<_, _> = vec.iter()
                                .map(|x| x * 2)
                                .enumerate()
                                .collect();
    println!("{:?}", map);
}