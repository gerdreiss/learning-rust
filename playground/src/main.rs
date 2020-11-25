use std::collections::HashMap;

fn main() {
    let mut split = "/path?a=b&c=&d===&d=ddd&c=ccc&empty=&".splitn(2, '?');
    let p = split.next().unwrap();
    let q = split.next().unwrap();
    println!("path = {}", p);
    println!("query = {}", q);

    let queries: HashMap<&str, &str> = q
        .split('&')
        .filter(|v| !v.is_empty())
        .map(|s| s.splitn(2, '='))
        .map(|mut s| (s.next(), s.next()))
        .filter(|(maybe_name, _)| maybe_name.is_some())
        .map(|(maybe_name, maybe_value)| {
            (
                maybe_name.unwrap_or_default(),
                maybe_value.unwrap_or_default(),
            )
        })
        .collect();

    println!("Queries: {:?}", queries);
}
