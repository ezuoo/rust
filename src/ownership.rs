pub fn ownership() {
  println!("This is ownership");

  // let mut str = String::from("hello");

  // str.push_str(", world");

  // println!("{str}");

  // let s1 = String::from("hello");
  // let s2 = s1;

  // println!("{}, world!", s1);
  // error[E0382]: borrow of moved value: `s1` >> copy trait
  // println!("{}, world!", s2);

  let s = String::from("hello"); // s가 스코프 안으로 들어옴.

  takes_ownership(s); // s의 값이 함수 안으로 이동.
                      // println!("{s}"); // error[E0382]: borrow of moved value: `s`

  let x = 5; // x가 스코프 안으로 들어옴.
  makes_copy(x); // x가 함수 안으로 이동했습니다만,
                 // i32는 Copy가 되므로, x를 이후에 계속
                 // 사용해도 됨.
} // 여기서 x는 스코프 밖으로 나가고, s도 그 후 나갑니다. 하지만 s는 이미 이동되었으므로,
  // 별다른 일이 발생하지 않음.

fn takes_ownership(some_string: String) {
  // some_string이 스코프 안으로 들어옴.
  println!("{}", some_string);
} // 여기서 some_string이 스코프 밖으로 벗어났고 `drop`이 호출됩니다. 메모리는
  // 해제됨.

fn makes_copy(some_integer: i32) {
  // some_integer이 스코프 안으로 들어옴.
  println!("{}", some_integer);
} // 여기서 some_integer가 스코프 밖으로 벗어났습니다. 별다른 일은 발생하지 않음.
