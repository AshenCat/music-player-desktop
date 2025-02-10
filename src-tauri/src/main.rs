// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    fs::{self, File},
    io::BufReader,
    sync::{Arc, Mutex},
    thread,
};

use model::Song;
use rodio::{Decoder, OutputStream, Sink};
use tauri::State;

mod model;

pub struct AppState {
    current_song: Mutex<Option<Arc<Sink>>>,
}

#[tauri::command]
fn get_songs() -> Vec<Song> {
    let mut mp3_files = Vec::new();

    println!("==================PARENT FOLDER");
    let one_below = fs::read_dir("../").unwrap();

    for path in one_below {
      println!("Name: {}", path.unwrap().path().display())
    }

    println!("==================ASSETS FOLDER");
    let entries = fs::read_dir("../assets").unwrap();

    for path_entries in entries {
      println!("Name: {}", path_entries.unwrap().path().display())
    }

    println!("==================");

    
    let entries = fs::read_dir("../assets").unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    let song = Song {
                        title: file_name_str.to_string(),
                    };
                    mp3_files.push(song);
                }
            }
        }
    }
    mp3_files
}

#[tauri::command]
fn play_song(title: String, state: State<'_, Arc<AppState>>) {
    let path = format!("../assets/{}", title);
    let state = state.inner().clone();

    thread::spawn(move || {
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error opening file {} : {}", path, e);
                return;
            }
        };

        let (_stream, stream_handle) = match OutputStream::try_default() {
            Ok(output) => output,
            Err(e) => {
                eprintln!("Error initializing output stream: {}", e);
                return;
            }
        };

        let sink = match Sink::try_new(&stream_handle) {
            Ok(sink) => Arc::new(sink),
            Err(e) => {
                eprintln!("Error creating sink: {}", e);
                return;
            }
        };

        match Decoder::new(BufReader::new(file)) {
            Ok(source) => sink.append(source),
            Err(e) => {
                eprintln!("Error decoding audio file: {}", e);
                return;
            }
        };

        {
            let mut current_song = state.current_song.lock().unwrap();
            if let Some(ref current) = *current_song {
                current.pause();
            }

            *current_song = Some(sink.clone())
        }

        
        println!("Now playing: {}", title);

        sink.set_volume(1.0);
        sink.sleep_until_end();
    });
}

#[tauri::command]
fn pause_song(state: State<'_, Arc<AppState>>) {
    let mut current_song = state.current_song.lock().unwrap();

    if let Some(ref sink) = *current_song {
        sink.pause();
    }

    println!("Song paused");
}

#[tauri::command]
fn set_volume(vol: f32, state: State<'_, Arc<AppState>>) {
    let mut current_song = state.current_song.lock().unwrap();

    if let Some(ref sink) = *current_song {
        sink.set_volume(vol);
    }

    println!("Volume set to: {}", vol);
}

fn main() {
    // app_lib::run();
    tauri::Builder::default()
        .manage(Arc::new(AppState {
            current_song: Mutex::new(None),
        }))
        .invoke_handler(tauri::generate_handler![
            get_songs, play_song, pause_song, set_volume
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
