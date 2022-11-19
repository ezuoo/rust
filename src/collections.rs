pub fn collections_main() {
  println!("## Collections ##");

  vector();
}

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
