fn main() {
    let mut x;
    // this access would be illegal, nowhere to draw the flow from:
    // assert_eq!(x, 42);
    x = 42;
    // this is okay, can draw a flow from the value assigned above:
    let y = &x;
    // this establishes a second, mutable flow from x:
    x = 43;
    // this continues the flow from y, which in turn draws from x.
    // but that flow conflicts with the assignment to x!
    // assert_eq!(*y, 42);

    // println!("{}", x);
    // println!("{}", y);
}
