fn solution(phrase: &str) -> String {
    phrase.chars().rev().collect()
}

fn main() {
  println!("{}", solution("world"));
}