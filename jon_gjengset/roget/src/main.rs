const GAMES: &str = include_str!("../data/answers.txt");

fn main() {
    let guesser = Naive::new();
    for answer in GAMES.split_whitespace() {
        play(answer, guesser);
    }
}
