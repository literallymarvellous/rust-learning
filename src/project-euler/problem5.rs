fn smallest_multiple(limit: i128) -> i128 {
  let mut num = 0i128;
  let mut count = 1i128;
  let mut found = false;

  loop {
    count += 1;

    num = limit * count;

    for i in 1..limit {
      if num % i == 0 {
        found = true;
      } else {
        found = false;
        break;
      }
    }

    if found {
      break;
    }
  }

  num
}

fn main() {
  println!("{}", smallest_multiple(20))
}