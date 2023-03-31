// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
mod person_logger;
use person_logger::PersonLogger;

const TARGET_FILE: &'static str = "../app.log";
// so basically a little bit of context before i forget
// this is a test to see how can i efficiently write to files and stuff
// so for now i wanna see the frontend send a request here, serialize the answer into an object and
// then write this object somewhere on disk (preferably in the file dir)
// so here i basically get a req from the front and write the file
#[tauri::command]
fn accept_person_data(name: String, age: i32, timestamp: String, comment: String) -> String {
    let logger: PersonLogger = PersonLogger::new((name, age, timestamp, comment), TARGET_FILE.to_owned());
    logger.flush().unwrap();
    return format!("{}", logger);
}

// TODO complete writing this one (like actually write to the file!()
// using the OpenOptions struct)
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![accept_person_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[cfg(test)]
mod tests {
    #[test]
    fn uh_huh() {
        let smth = 4;
        assert_eq!(smth, 4);
    }
}
