use itertools::Itertools;

fn word_groupings(words: Vec<String>) -> Vec<Vec<String>> {
    words
        .into_iter()
        .group_by(|word| word.chars().sorted().collect::<String>())
        .into_iter()
        .map(|(_, group)| group.collect_vec())
        .collect()
}

pub(crate) fn print_group_for_word() {
    let words: Vec<String> = ["the", "teh", "het", "stupid", "studpi", "apple", "appel"]
        .into_iter()
        .map(|x| x.to_string())
        .collect();

    println!("Words:");
    println!("{:?}", &words);

    let groupings = word_groupings(words);

    println!("Groupings:");
    for group in &groupings {
        println!("{:?}", group);
    }

    let input = String::from("het");

    let group = groupings.into_iter().find(|group| group.contains(&input));

    println!("Found group for word `{}`:\n{:?}", input, &group)
}
