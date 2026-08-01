
use std::{io::stdin, println};
use crate::components::note::{Note};

enum MENU {
    Browse,
    Create,
    Update,
    Delete,
}

fn menu_list() {
    println!(" 1. Browse \n 2. Create \n 3. Edit \n 4. Delete \n 0. Exit");
}

pub fn show(){
    let mut  opt : String = String::new();


    menu_list();

    stdin().read_line( &mut opt).expect("Unable to read input");

    let choosen = match opt.trim() {
        "1" => MENU::Browse,
        "2" => MENU::Create,
        "3" => MENU::Update,
        "4" => MENU::Delete,
        _ => {
            println!("Unkown input. Defaulting to browse");
            MENU::Browse
        }
    };

    menu_descriptions(choosen);
    
}

fn menu_descriptions(choice : MENU) {
    match choice {
        MENU::Browse => {
            println!("Looking for any existing notes ....")
        }


        MENU::Create => {

            let mut title = String::new();
        
            let mut body = String::new();

            println!("Enter title of the Note");
            stdin().read_line(&mut title).expect("Failed to read input");

            println!("What's the content of the Note?");
            stdin().read_line(&mut body).expect("Failed to read input");

            // let mut notes : Vec<Note> = Vec::new();

            Note::create_note(title, body);

            println!("Creating a new note ....")
        }


        MENU::Update => {
            println!("Updating a Note, Please wait ....")
        }
        MENU::Delete => {
            println!("Sorry! Coulld not delete note ....")
        }
    }
}