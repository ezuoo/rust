// https://rinthel.github.io/rust-lang-book-ko/ch03-01-variables-and-mutability.html
pub fn variables() {
  // let x = 5;
  // error[E0384]: cannot assign twice to immutable variable `x`

  let mut x = 5;
  println!("The value of x is: {x}");
  x = 6;
  println!("The value of x is: {x}");
  // #################################################################

  const MAX_POINTS: u32 = 100_000;

  println!("Max points : {MAX_POINTS}");

  // #################################################################

  let shadowed_values = 5;
  let shadowed_values = shadowed_values + 1;
  let shadowed_values = shadowed_values * 2;

  println!("Shadowed_values is: {}", shadowed_values);

  // #################################################################

  // let mut spaces = "   ";
  // spaces = spaces.len();
  // error[E0308]: mismatched types

  let spaces = "   ";
  let spaces = spaces.len();
  println!("Spaces is: {}", spaces);

  // #################################################################
}
