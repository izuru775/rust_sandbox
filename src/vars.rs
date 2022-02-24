// Variables hold primitive data or reference to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run(){
    let name = "Sodi";
    let mut age = 30;
    println!("My name is {} and I'am {}",name,age);
    age = 31;
    println!("My name is {} and I'am {}",name,age);

    // Define constant
    const ID:i32 = 001;
    println!("ID : {}",ID);

    // Assign multiple vars
    let(my_name, my_age) = ("Sodi",30);
    println!("{} is {}",my_name, my_age);
}