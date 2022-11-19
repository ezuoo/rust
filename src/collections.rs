use std::collections::HashMap;

pub fn collections_main() {
  println!("## Collections ##");

  // vector();
  // string();
  hash_map();
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

/*

*/
fn hash_map() {
  println!("# Hash Map #");

  /////////////////// create empty hash map ///////////////////
  // let mut scores = HashMap::new();
  // scores.insert(String::from("Blue"), 10);
  // scores.insert(String::from("Yellow"), 50);

  // let teams = vec![String::from("Blue"), String::from("red")];
  // let initial_scores = vec![10, 20];
  // let anoter_scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

  // for (k, v) in &anoter_scores {
  //   println!("{k} : {v}");
  // }

  // println!("anoter_scores : {:?}", anoter_scores);

  /////////////////// Insert only if there is no value assigned to the update key  ///////////////////
  // let mut scores = HashMap::new();
  // scores.insert(String::from("Blue"), 10);

  // scores.entry(String::from("Yellow")).or_insert(50);
  // scores.entry(String::from("Blue")).or_insert(50);

  // println!("{:?}", scores);

  /////////////////// Updating values based on old values  ///////////////////
  let text = "hello world wonderful world";
  let mut map = HashMap::new();

  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }

  println!("{:?}", map);
}
