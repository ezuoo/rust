pub fn collections_main() {
  println!("## Collections ##");

  // vector();
  // string();
}

/*
fn vector() {
  println!("# Vector #");
  let vector: Vec<i32> = Vec::new();
  let mut v1 = Vec::new();
  let v2 = vec![1, 2, 3];

  v1.push(4);
  v1.push(5);
  v1.push(6);
  v1.push(7);

  println!("vector : {:?}", vector);
  println!("v1 : {:?}", v1);
  println!("v2 : {:?}", v2);

  // read vector elements

  let read_vector1: &i32 = &v2[2];
  // let read_vector2: Option<&i32> = v2.get(2);
  let read_vector3 = v2[0];

  println!("v2 : {:?}", v2);
  println!("read_vector1 : {read_vector1}");
  println!("read_vector2 : ");
  println!("read_vector3 : {read_vector3}");

  for i in &mut v1 {
    *i += 50;
  }

  v1.pop();
  println!("v1 : {:?}", v1);
}
 */

/*
fn string() {
  println!("# String #");

  //////////// create empty string ////////////
  // let mut string = String::new();

  //////////// string literal to string ////////////
  // let string_literal = "initial contents";
  // let string_from_string_literal = String::from(string_literal);
  // let string_literal_to_string = string_literal.to_string();
  // let string_literal_to_string = "initial contents".to_string();

  //////////// renew string ////////////
  // let mut string = String::from("string");
  // let new_string = "new string";

  // string.push_str(&new_string);

  // println!("string : {}", string);
  // println!("new_string : {}", new_string);

  // let s1 = String::from("hello, ");
  // let s2 = String::from("world !");
  // let s3 = s1 + &s2;
  // println!("s3 : {s3}")

  //////////// string index ////////////
  // let s1 = String::from("hello");

  // error[E0277]: the type `String` cannot be indexed by `{integer}`
  // let h = s1[0];

  // println!("s1 length : {}", s1.len());
  // println!("이주호 length : {}", "이".len());
}
*/
