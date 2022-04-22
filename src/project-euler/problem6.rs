fn sum_of_squares(n: i128) -> i128 {
  let mut sum = 0i128;

  for i in 1..=n {
    sum += i * i;
  }

  sum
}

fn square_of_sum(n: i128) -> i128 {
  let mut sum = 0i128;

  for i in 1..=n {
    sum += i;
  }

  sum * sum
}

fn sum_square_difference(n: i128) -> i128 {
  let sum_of_square = sum_of_squares(n);
  let square_of_sum = square_of_sum(n);

  square_of_sum - sum_of_square
}

fn main() {
  println!("{}", sum_square_difference(100))
}