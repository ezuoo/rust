// https://rinthel.github.io/rust-lang-book-ko/ch03-03-how-functions-work.html
pub fn functions(argu: u32) -> u32 {
  println!("The value of arguments : {argu}");

  argu * 2
  // argu * 2;
  // error[E0308]: mismatched types
}
