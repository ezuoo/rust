// struct User {
//   username: String,
//   email: String,
//   sign_in_count: u64,
//   active: bool,
// }
#[derive(Debug)]
struct Rectangle {
  length: u64,
  width: u64,
}

impl Rectangle {
  fn area(&self) -> u64 {
    self.length * self.width
  }

  fn square(size: u64) -> Rectangle {
    Rectangle {
      length: size,
      width: size,
    }
  }
}

pub fn structs_main() {
  println!("## Struct ##");
  ///////////////////////// 1 ////////////////////////////
  // make_instance();
  // let user_instance = build_instance(String::from("test@test.com"), String::from("tester"));
  ///////////////////////// 2 ////////////////////////////
  // let rect = Rectangle {
  //   length: 5,
  //   width: 10,
  // };

  // println!("The area of rectangle : {}", area(&rect));
  // println!("The area of rectangle : {:?}", rect);
  ///////////////////////// 3 ////////////////////////////
  // let rect = Rectangle {
  //   length: 5,
  //   width: 10,
  // };

  // println!("The area of rectangle : {}", rect.area());
  ///////////////////////// 4 ////////////////////////////
  let new_rect = Rectangle::square(10);
  println!("The rectangle data is {:?}", new_rect);
  println!("The area of rectangle : {}", new_rect.area());
}

// fn area(rect: &Rectangle) -> u64 {
//   rect.width * rect.length
// }

// fn build_instance(email: String, username: String) -> User {
//   User {
//     email,
//     username,
//     sign_in_count: 1,
//     active: true,
//   }
// }

// fn make_instance() {
//   let user1 = User {
//     username: String::from("james"),
//     email: String::from("james@gmail.com"),
//     sign_in_count: 1,
//     active: true,
//   };

//   println!("{}", user1.username);
//   println!("{}", user1.email);
//   println!("{}", user1.sign_in_count);
//   println!("{}", user1.active);
// }
