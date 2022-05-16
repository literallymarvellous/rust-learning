fn gimme(input_array: [i32;3]) -> () {
  let mut vec_arr = input_array.to_vec();
  vec_arr.sort();
  input_array.iter().position(|&x| x == vec_arr[1]).unwrap();
}


fn main() {
  gimme([2, 3, 1]);
  // gimme([5, 10, 14]);
}