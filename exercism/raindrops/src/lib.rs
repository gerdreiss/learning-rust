pub fn raindrops(n: u32) -> String {
    let result = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .iter()
        .filter(|(k, _)| n % *k == 0)
        .map(|(_, v)| v)
        .fold(String::new(), |acc, v| format!("{}{}", acc, v));

    if result.is_empty() {
        n.to_string()
    } else {
        result.to_owned()
    }
}
