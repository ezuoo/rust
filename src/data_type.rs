pub fn data_type() {
  // let guess = "42".parse().expect("Not a number!");
  // error[E0282]: type annotations needed

  let guess: u32 = "42".parse().expect("Not a number!");
  println!("The value of variable : {guess}");

  let tuple: (i32, f64, u8) = (123, 1.1, 5);
  println!("The value of tuple : {:?}", tuple);

  // let x: [i32; 3] = [1, 2, 3];
  let array = [1, 2, 3];
  println!("The value of array : {:?}", array);
}
