fn main() {
    let mut name = String::from("Samwel");
    println!("Hello, there!");
    println!("Hello, {}!", &name);
    
    name = String::from("Developer");
    println!("Your name is, {}!", &name);
}

