fn largest_palindrome_from_3_digit_numbers() -> i128 {
  let mut largest_palindrome: i128 = 0;
  for i in 100..1000 {
    for j in 100..1000 {
      let num: i128 = i * j;
      let mut num_string = num.to_string();
      let mut num_reverse: String = num_string.chars().rev().collect();
      if num_reverse.parse::<i128>().unwrap() == num_string.parse::<i128>().unwrap() && num > largest_palindrome {
        largest_palindrome = num;
      }
    }
  }
  return largest_palindrome;
}

fn main() {
  println!("largest Palindrome {}", largest_palindrome_from_3_digit_numbers());
}