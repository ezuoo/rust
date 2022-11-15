pub fn slices_main() {
  println!("## slices ##");
  ///////////////////////////// 1 ////////////////////////////
  // let mut s = String::from("hello world");

  // println!("The length of '{}' first word : {}", s, first_word(&s));

  // s.clear();
  ///////////////////////////// 2 ////////////////////////////
  let s = String::from("hello world");

  let hello = &s[0..5];
  let world = &s[6..11];
  println!("{hello} {world}");

  let s1 = &s[..5];
  let s2 = &s[6..];
  println!("{s1} {s2}");

  let s3 = &s[..];
  println!("{s3}");
}

// fn first_word(str: &String) -> usize {
//   let bytes = str.as_bytes();

//   for (i, &item) in bytes.iter().enumerate() {
//     if item == b' ' {
//       return i;
//     }
//   }

//   str.len()
// }
