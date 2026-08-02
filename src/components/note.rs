use std::{
    env::{self, var}, format, fs::{self}, println,
};

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use time::OffsetDateTime;

use crate::components::note;

#[derive(Serialize, Deserialize)]
pub struct Note {
    id: u32,
    title: String,
    body: String,
    created_at: String,
    updated_at: String,
}

fn save(notes: &Vec<Note>) {
    let file_name = env::var("JSON_STORAGE_FILE")
                                .expect("Sorry! Storage file not specified");

    let path = format!("storage/{file_name}");//STRING CONCATENATION YA KAWAIDA INAKATAA HAPA

    let json_text = to_string_pretty(notes)
                            .expect("Unable to Convert to Json");

    fs::write(path, json_text)
        .expect("Unable to save Note");
}

impl Note {
    pub fn create_note(title: String, body: String) {
        let file_name = env::var("JSON_STORAGE_FILE")
                                    .expect("Sorry! Storage file not specified");//CRASH APP NZIMA NA RUDISHA HIO MESSAGE

        let path = format!("storage/{file_name}");

        let mut notes: Vec<Note> = match fs::read_to_string(&path) {
            Ok(content) => from_str(&content)//HII OK MAANA YAKE TUBADILI KILICHOPO KWENYE NOTES.JSON KUJA  KUWA JSON
                                   .unwrap_or(Vec::new()), // KAMA NI EMPTY ALLOCATE HEAP YA KUTUNZA NEW DATA(NOTES)
            Err(_) => Vec::new(),
        };

        //Ninahitaji simple auto-incrementing ID ili iwe rahisi kuSelect note ipi ya kufuta au kuEdit
        let id = notes.iter()//LOOP THROUGH ARRAY IN NOTES.JSON
                            .map(|n| n.id)//CONSIDER ID's TU
                            .max()//TAKE ID KUBWA KULIKO ZOTE
                            .unwrap_or(0) + 1;  // CHUKUA ID KUBWA KULIKO KISHA JUMLISHA '1' KAMA HAMNA YOYOTE CHUKUA '0+1'

        let now = OffsetDateTime::now_utc()
                                         .to_string(); // HII INATUPA TIMESTAMPS FORMAT NZURI KAMA LARAVEL

        let new_note = Note {
            id,
            title,
            body,
            created_at: now.clone(),
            updated_at: now,
        };

        notes.push(new_note);
        save(&notes);

        println!("Creating a new note ....");
        println!("DONE")    
    }

    pub fn display_notes() {
        let file_name = env::var("JSON_STORAGE_FILE")
                                    .expect("Sorry! Storage file not specified");

        let path = format!("storage/{file_name}");

        let contents = match fs::read_to_string(path) {
            Ok(data) => data,
            Err(_) => {
                println!("Could not convert to json string");
                return;
            }
        };

        let notes: Vec<Note> = from_str(&contents)
                               .unwrap_or(Vec::new());

        println!(
            "****************************** ALL NOTES ({}) *******************************",
            notes.len()
        );

        for note in notes {
            println!("---------------------------------------------------------");
            println!("---------------------------------------------------------");
            println!("ID: \t {}", note.id);
            println!("TITLE: \t {}", note.title);
            println!("CONTENT: \t {}", note.body);
            println!("DATE CREATED: \t {}", note.created_at);
            println!("---------------------------------------------------------");
            println!("---------------------------------------------------------\n \n");
        }
    }

    pub fn update(id : u32, title : Option<String>, body : Option<String>) {
        let file_name = var("JSON_STORAGE_FILE").expect("Sorry! Storage file not specified");
        let path = format!("storage/{file_name}");
        let content = fs::read_to_string(path).expect("Could not read file");

        let mut notes : Vec<Note> = from_str(&content).expect("Could not process file");

        let mut is_found = false;

        for note in &mut notes {
            if id == note.id {

                is_found = true;

                if let Some(new_title) = title {
                    note.title = new_title;
                }
                
                if let Some(new_body) = body {
                    note.body = new_body;
                }

                note.updated_at = OffsetDateTime::now_utc().to_string();

                println!("The Note with the ID {} was Updated Successfully", &id);
                println!("******************** THE UPDATED NOTE *****************");
                println!("ID: \t{}", note.id);
                println!("TITLE: \t{}", note.title);
                println!("BODY: \t{}", note.body);
                break;
            }
        }

        if !is_found {
            println!("A Note with the given ID was not found")
        }

    }

    // fn delete(&self) {
    //     println!(
    //         "Do you want to DELETE the Note with the title '{}'?",
    //         self.title
    //     )
    // }
}
