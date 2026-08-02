mod components;

use components::menu::show;
use dotenvy::dotenv;

fn main() {
    dotenv().ok();//LOAD .ENV VARIABLES ZOTE
    println!("Hey there! What's up?");
    println!("What do you wanna do?");

    show();//DISPLAY MENU
}
