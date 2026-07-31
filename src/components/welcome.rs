enum MENU {
    Browse,
    Create,
    Update,
    Delete,
}


fn greetings() {
    println!("Hey there! What's up?");
    println!("What do you wanna do?")
}


fn menu_descriptions() {
    match MENU {
        MENU::Browse => {
            println!("Looking for any existing notes ....")
        }
        MENU::Create => {
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
