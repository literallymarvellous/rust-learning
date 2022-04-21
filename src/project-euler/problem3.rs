
fn is_prime(num: i128) -> bool {
  let s = (num as f64).sqrt() as i128;
  for i in 2..=s{
      if num % i == 0 {
        return false;
      }
  }
  true
}

fn largest_prime_factor(num: i128) -> Vec<i128> {
    let mut factors = Vec::new();
    for i in 1..=num {
      if num % i == 0 {
          if factors.contains(&i) {
            return factors;
          }
          if is_prime(i) {
            factors.push(num / i);
            factors.push(i);
          }
      }
    }
    factors
}

fn main() {
  println!{"factors: {:?}", largest_prime_factor(600851475143)}
}