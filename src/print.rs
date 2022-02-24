pub fn run (){
    // Print to console
    println!("Hello, world!");

    // Basic formatting
    println!("{} is from {}", "Sodi", "Sydney" );

    // Positional Arguments
    println!("{0} is form {1} and {0} likes to {2}","Sodi","Sydney","code");

    // Named Arguments
    println!("{name} like to play {activity}",name ="Sodi",activity="football");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}",10,10,10);

    // Placeholder for debug trait
    println!("{:?}",(12,true,"Hello"));

    // Basic math
    println!("10 + 10 = {}",10 +10);
}