#[derive(Debug)]
pub struct StringHolder<'a> {
    s: &'a str,
    t: &'a str,
}

pub fn two_strings<'a>(s: &'a str, t: &'a str) -> StringHolder<'a> {
    StringHolder { s, t }
}

fn main() {
    let mut s = make_str(7);

    // you can mutate s here
    s.push_str("people");

    s = to_people(s);
    to_frogs(&mut s);

    let p = part(&s);

    // but you can't mutate s here anymore
    //s.push_str("p");

    println!("p = {}", p);
    println!("s = {}", s);

    let tog = two_strings(p, &s);

    println!("tog = {:?}", tog);
}

fn to_people(mut s: String) -> String {
    s.push_str(" people");
    s
}

fn to_frogs(s: &mut String) {
    s.push_str("frogs");
}

/// &str lives on the stack while String lives on the heap
fn make_str(n: i32) -> String {
    let s = format!("hello {} ", n);
    s
}

fn part<'a>(s: &'a str) -> &'a str {
    if s.len() > 4 {
        &s[0..4]
    } else {
        s
    }
}
