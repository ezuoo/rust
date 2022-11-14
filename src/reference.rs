pub fn reference_main() {
  let mut s1 = String::from("hello");

  // let len = calculate_length(&s1);

  // println!("The length of '{}' is {}.", s1, len);

  // change(&s1);
  // error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference

  // change(&mut s1);

  // 불변 참조자와 가변 참조자는 동시에 쓸 수 없음
  // error[E0596]: cannot borrow `s2` as mutable, as it is not declared as mutable
  // let s2 = &s1;
  // let s3 = &s2;
  // let s4 = &mut s2;
}

// fn change(some_string: &mut String) {
//   some_string.push_str(", world");
// }

// fn calculate_length(s: &String) -> usize {
//   s.len()
// }
