pub fn ownership() {
  println!("## This is ownership ##");

  // ////////////////////////////////1//////////////////////////////////
  // let mut str = String::from("hello");

  // str.push_str(", world");

  // println!("{str}");

  // /////////////////////////////////2/////////////////////////////////
  // let s1 = String::from("hello");
  // let s2 = s1;
  // let s2 = s1.clone(); // Deep Copy

  // println!("{}, world!", s1);
  // error[E0382]: borrow of moved value: `s1` >> copy trait
  // println!("{}, world!", s2);
  // /////////////////////////////////3/////////////////////////////////
  // let s = String::from("hello"); // s가 스코프 안으로 들어옴.

  // takes_ownership(s); // s의 값이 함수 안으로 이동.
  // println!("{}", s); // error[E0382]: borrow of moved value: `s`

  // let x = 5; // x가 스코프 안으로 들어옴.
  // makes_copy(x); // x가 함수 안으로 이동했습니다만,
  // //////////////////////////////// 4 //////////////////////////////////
  // let s1 = given_ownership();
  // let s2 = String::from("hello");
  // let s3 = taken_and_given_ownership(s2);
  // println!("{}", s1);
  // println!("{}", s2);
  // println!("{}", s3);
  // //////////////////////////////// 5 //////////////////////////////////
  let str = String::from("hello");
  println!("length of '{}' : {}", str, str.len());
}

// fn given_ownership() -> String {
//   let str = String::from("hello");
//   str
// }

// fn taken_and_given_ownership(some_string: String) -> String {
//   some_string
// }

// fn takes_ownership(some_string: String) {
// some_string이 스코프 안으로 들어옴.
// println!("{} << in fn takes_ownership", some_string);
// } // 여기서 some_string이 스코프 밖으로 벗어났고 `drop`이 호출됩니다. 메모리는
// 해제됨.

// fn makes_copy(some_integer: i32) {
//   // some_integer이 스코프 안으로 들어옴.
//   println!("{}", some_integer);
// } // 여기서 some_integer가 스코프 밖으로 벗어났습니다. 별다른 일은 발생하지 않음.
