use crate::components::note::Note;
use std::{io::stdin, println};

enum MENU {
    Browse,
    Create,
    Update,
    Delete,
}

fn menu_list() {
    println!(" 1. Browse \n 2. Create \n 3. Edit \n 4. Delete \n 0. Exit");
}

pub fn show() {
    let mut opt: String = String::new();//ALOCATE TO HEAP SABABU KATIKA HII STAGE YA APP BADO HATUJUI THE SIZE YA BYTES NEEDED TO STORE OUR INPUT

    menu_list();

    stdin().read_line(&mut opt).expect("Unable to read input");

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

    menu_actions(choosen);
}

fn menu_actions(choice: MENU) {
    match choice {
        MENU::Browse => {
            println!("Loading ....");
            Note::display_notes();
            println!("DONE");
        }

        MENU::Create => {
            let mut title = String::new();

            let mut body = String::new();

            println!("Enter title of the Note");
            stdin().read_line(&mut title).expect("Failed to read input");
            title = title.trim().to_string();//TRIM VALUE KUONDOA '\n' NA WHITESPACES

            println!("What's the content of the Note?");
            stdin().read_line(&mut body).expect("Failed to read input");
            body = body.trim().to_string();//TUKISHA TRIM TU VALUE INARUDI KAMA &str LAKINI  CREATE_NOTE() INA EXPECT String

            Note::create_note(title, body);
        }

        MENU::Update => {
            let mut note_selected = String::new();
            println!("Please enter the ID of the Note you want to change");
            stdin().read_line(&mut note_selected).expect("Failed to read input");

            let id : u32 = match note_selected.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid ID");
                    return;
                }
            };
            
            let mut title = String::new();
            println!("Please provide a new Title");
            stdin().read_line(&mut title).expect("Failed to read input");
            title = title.trim().to_string();
            
            let mut body = String::new();
            println!("Now you can change the Note Content");
            stdin().read_line(&mut body).expect("Failed to read input");
            body = body.trim().to_string();

            Note::update(id, Some(title), Some(body));
        }
        
        MENU::Delete => {
            let mut id = String::new();
            println!("Enter ID for the note you want to delete");
            stdin().read_line(&mut id).expect("Could not read your selection");

            match id.trim().parse(){
                Ok(id) =>{ 
                    Note::delete(id);
                },
                Err(_) => println!("Please enter a valid ID")
            };
        }
    }
}
