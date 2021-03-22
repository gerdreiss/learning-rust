use crate::three_vowels::three_vowels;
use crate::three_vowels::three_vowels_functional;

mod three_vowels;

fn main() {
    let ferris = "Ferris".to_string();
    let curious = "Curious".to_string();
    let sentence = "Once upon a time, there was a friendly curious crab named Ferris".to_string();

    println!("{}: {}", ferris, three_vowels(&ferris));
    println!("{}: {}", curious, three_vowels(&curious));
    println!("{}: {}", sentence, three_vowels(&sentence));

    println!("{}: {}", ferris, three_vowels_functional(&ferris));
    println!("{}: {}", curious, three_vowels_functional(&curious));
    println!("{}: {}", sentence, three_vowels_functional(&sentence));

    // This works fine, but the following two lines would fail:
    // println!("Ferris: {}", three_vowels("Ferris"));
    // println!("Curious: {}", three_vowels("Curious"));
}
