fn main() {
    print_iter((&["hello", "world"]).into_iter());
    print_iter(&["hello".to_string(), "world".to_string()]);
    print_iter(vec!["hello", "world"]);
    print_iter(&vec!["hello".to_string(), "world".to_string()]);
}

fn print_iter<S: AsRef<str>, I: IntoIterator<Item = S>>(v: I) {
    println!("--------");
    for (i, val) in v.into_iter().enumerate() {
        println!("  {} = {}", i, val.as_ref());
    }
}
