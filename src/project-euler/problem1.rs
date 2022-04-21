fn multiples_of_3_and_5(limit: u64) -> u64 {
  let mut total: u64 = 0;
  for i in 1..limit {
    if i % 3 == 0 || i % 5 == 0 {
      total += i;
      println!("i : {}, total : {}", i, total);
    }
  }
  total
}

fn main() {
  println!("total: {}", multiples_of_3_and_5(1000));
}