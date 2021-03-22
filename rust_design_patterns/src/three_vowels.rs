pub fn three_vowels(word: &String) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true;
                }
            }
            _ => vowel_count = 0,
        }
    }
    false
}

pub fn three_vowels_functional(word: &String) -> bool {
    return word
        .chars()
        .fold(Vec::new(), |mut acc, c| match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                acc.push(c);
                return acc;
            }
            _ => {
                if acc.len() >= 3 {
                    acc
                } else {
                    Vec::new()
                }
            }
        })
        .len()
        >= 3;
}
