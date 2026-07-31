pub struct Note {
    id : u32,
    title : String,
    body : String,
    created_at : String,
    updated_at : String
}

impl Note {
    fn create(){
        println!("You are creating a Note")
    }

    fn update(&self) {
        println!("The Note with the title '{}' is to be edited", self.title)
    }

    fn delete(&self) {
        println!("Do you want to DELETE the Note with the title '{}'?", self.title)
    }
}