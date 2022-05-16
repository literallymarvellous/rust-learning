fn high_and_low(numbers: &str) -> String  {  
    let mut vec_arr: Vec<&str> = numbers.split(" ").collect();
    println!("{:?}", vec_arr);
    vec_arr.sort_by(|a, b| a.parse::<i32>().unwrap().cmp(&b.parse::<i32>().unwrap()));
    println!("{:?}", vec_arr);
    vec_arr[vec_arr.len() - 1].to_owned() + " " + vec_arr[0]
}

fn main() {
  println!("{}", high_and_low("1 2 3 4 5"));
}