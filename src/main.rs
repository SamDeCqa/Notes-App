mod components;

use components::menu::show;
use dotenvy::dotenv;

fn main() {
    dotenv().ok();
    println!("Hey there! What's up?");
    println!("What do you wanna do?");

    show();
}
