// mod first_cargo;
// mod guess_number;
// mod variables;
// mod data_type;
mod functions;
fn main() {
    // println!("Hello, world!");

    // first_cargo::first_cargo();
    // guess_number::guess_number();
    // variables::variables();
    // data_type::data_type();
    let return_value = functions::functions(32);
    println!("The value of function's return : {return_value}");
}
