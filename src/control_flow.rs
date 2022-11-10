// pub fn control_flow(argu: u32) -> u32 {
pub fn control_flow() {
  // if argu < 5 {
  //   argu
  // } else {
  //   0
  // }

  // loop {
  //   println!("again!");
  // }

  let a = [10, 20, 30, 40, 50];
  println!("{:?}", a.iter());
  for element in a.iter() {
    println!("the value is: {}", element);
  }

  for number in (1..=4).rev() {
    println!("{}!", number);
  }
  println!("LIFTOFF!!!");
}
