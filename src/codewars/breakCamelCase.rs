fn solution(s: &str) -> () {
    let mut first_string = String::from("");
    for i in s.chars() {
        if i.is_uppercase() {
          first_string  = first_string + " "
        }
        first_string += &i.to_string();
    }
    println!("{}", first_string);
}

fn main() {
  solution("camelCase");
  solution("identifier")
  solution("camelCaseTest");
}