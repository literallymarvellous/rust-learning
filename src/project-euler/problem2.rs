fn even_fibonacci_numbers() -> u128 {
  let mut a: u128 = 0;
  let mut b: u128 = 1;
  let mut c: u128;
  let mut sum: u128 = 0;

  loop {

    c = a + b;
    b = a;
    a = c;
    println!("c: {}", c);
    if c % 2 == 0 {
      sum += c;
    }

    if c >= 4000000 {
      break;
    }
  }

  sum
}

fn main() {
  println!("total : {}", even_fibonacci_numbers());
}