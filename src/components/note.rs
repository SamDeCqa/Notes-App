use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use time::OffsetDateTime;

#[derive(Serialize, Deserialize)]
pub struct Note {
    id: u32,
    title: String,
    body: String,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

fn save(notes: &Vec<Note>) {
    let json_text = to_string_pretty(notes).expect("Unable to Convert to Json");
    fs::write("Notes.json", json_text).expect("Unable to save Note");
}

impl Note {
    pub fn create_note(notes: &mut Vec<Note>, title: String, body: String) {
        //Ninahitaji simple auto-incrementing ID ili iwe rahisi kuSelect note ipi ya kufuta au kuEdit
        let id = notes.iter().map(|n| n.id).max().unwrap_or(0) + 1;
        let now = OffsetDateTime::now_utc();

        let new_note = Note {
            id: id,
            title,
            body,
            created_at: now,
            updated_at: now,
        };

        notes.push(new_note);
        save(notes);

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
