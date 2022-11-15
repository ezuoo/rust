pub fn reference_main() {
  println!("## Reference ##");
  //////////////////////////// 1 /////////////////////////////////
  // let s1 = String::from("hello");

  // let len = calculate_length(&s1);
  // println!("The length of '{}' is {}.", s1, len);
  //////////////////////////// 2 /////////////////////////////////
  // let mut s1 = String::from("hello");
  // println!("Before change : {}", s1);
  // change(&s1);
  // error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference

  // change(&mut s1);
  // println!("After change : {}", s1);
  //////////////////////////// 3 /////////////////////////////////
  // 불변 참조자와 가변 참조자는 동시에 쓸 수 없음
  // error[E0596]: cannot borrow `s2` as mutable, as it is not declared as mutable
  // let s2 = &s1;
  // let s3 = &s2;
  // let s4 = &mut s2;
  //////////////////////////// 4 /////////////////////////////////
  let dangle_str = dangle();
  println!("Dangle String : {}", dangle_str);
}

fn dangle() -> String {
  String::from("Hello dangle !")
}

// fn change(some_string: &mut String) {
//   some_string.push_str(", world");
// }

// fn calculate_length(s: &String) -> usize {
//   s.len()
// }
