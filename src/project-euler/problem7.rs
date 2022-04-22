fn is_prime(num: i128) -> bool {
  let s = (num as f64).sqrt() as i128;
  for i in 2..=s{
      if num % i == 0 {
        return false;
      }
  }
  true
}

fn prime(limit: i128) -> i128 {
  let mut primes = Vec::new();
  let mut n = 2;

  while primes.len() < limit as usize {
      if is_prime(n) {
        primes.push(n)
      }
      
      n += 1;
  }

  primes.pop().unwrap()
} 

fn main() {
  println!("{}", prime(10001))
}