use std::{env, format, fs};

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize)]
pub struct Note {
    id: u32,
    title: String,
    body: String,
    created_at: String,
    updated_at: String,
}

fn save(notes: &Vec<Note>) {
    let file_name =env::var("JSON_STORAGE_FILE").expect("Sorry! Storage file not specified");
        let path= format!("storage/{file_name}");
    let json_text = to_string_pretty(notes).expect("Unable to Convert to Json");
    
    fs::write(
        path,
        json_text,
    )
    .expect("Unable to save Note");
}

impl Note {
    pub fn create_note(title: String, body: String) {

        let file_name =env::var("JSON_STORAGE_FILE").expect("Sorry! Storage file not specified");
        let path= format!("storage/{file_name}");

        let mut notes: Vec<Note> = match fs::read_to_string(&path) {
            Ok(content) => from_str(&content).unwrap_or(Vec::new()),
            Err(_) => Vec::new(),
        };

        //Ninahitaji simple auto-incrementing ID ili iwe rahisi kuSelect note ipi ya kufuta au kuEdit
        let id = notes.iter().map(|n| n.id).max().unwrap_or(0) + 1;

        let now = OffsetDateTime::now_utc().to_string();

        let new_note = Note {
            id,
            title,
            body,
            created_at: now.clone(),
            updated_at: now,
        };

        notes.push(new_note);
        save(&notes);

        println!("You are creating a Note")
    }

    // fn update(&self) {
    //     println!("The Note with the title '{}' is to be edited", self.title)
    // }

    // fn delete(&self) {
    //     println!(
    //         "Do you want to DELETE the Note with the title '{}'?",
    //         self.title
    //     )
    // }
}
