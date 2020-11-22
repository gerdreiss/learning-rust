

fn add_4(x: i32) -> i32 {
    x + 4
}

fn maybe_add_4(y: Option<i32>) -> Option<i32> {
    y.map(add_4)
}

fn foo(input: Option<i32>) -> Option<i32> {
    input.filter(|x| x >= &0)
}

fn bar(input: Option<i32>) -> Result<i32, &'static str> {
    foo(input).ok_or("ErrNegative")
}

fn main() {
    println!("Some(1) + Some(4) = {:?}", maybe_add_4(Some(1))); 
    println!("Some(1).and(Some(2)) = {:?}", Some(1).and(Some(2)));   
    println!("Some(1).or(Some(2)) = {:?}", Some(1).or(Some(2)));
    println!("Some(1).and_then(|x| Some(x + 1)) = {:?}", Some(1).and_then(|x| Some(x + 1)));

}
