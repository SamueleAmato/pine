// File: src/json_manage.rs

extern crate serde;
extern crate serde_json;
use std::{fs, io};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};
use std::env;
use std::process::Command;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
struct TodoList {
    todo: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Config {
    editor: String,
}

pub fn add_task(task: &String) -> Result<(), Box<dyn Error>> {
    // Il nome del file JSON
    let filename = "todo.json";

    // Contenuto da aggiungere alla lista
    let task_name = task;

    // Apre il file JSON in lettura
    let mut file = File::open(filename)?;

    // Legge il contenuto del file JSON in una stringa
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserializza il contenuto del file JSON in una struttura dati
    let mut todo_list: TodoList = serde_json::from_str(&contents)?;

    // Aggiunge il contenuto della variabile tesst alla lista
    todo_list.todo.push(task_name.to_string());

    // Apre il file JSON in scrittura sovrascrivendo il contenuto esistente
    let mut file = OpenOptions::new().write(true).truncate(true).open(filename)?;

    // Serializza la struttura dati in formato JSON e la scrive nel file
    let new_contents = serde_json::to_string(&todo_list)?;
    file.write_all(new_contents.as_bytes())?;

    // Stampa la lista completa
    println!("Lista completa:");
    for task in &todo_list.todo {
        println!("{}", task);
    }

    Ok(())
}

pub fn delete_task(index: usize) -> Result<(), Box<dyn Error>> {
    let filename = "todo.json";
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut todo_list: TodoList = serde_json::from_str(&contents)?;
    if index < todo_list.todo.len() {
        todo_list.todo.remove(index);
        let mut file = OpenOptions::new().write(true).truncate(true).open(filename)?;
        let new_contents = serde_json::to_string(&todo_list)?;
        file.write_all(new_contents.as_bytes())?;
        println!("Task deleted successfully.");
    } else {
        println!("Invalid task index.");
    }
    Ok(())
}

pub fn list_tasks() -> Result<(), Box<dyn Error>> {
    let filename = "todo.json";
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let todo_list: TodoList = serde_json::from_str(&contents)?;
    println!("Todo list:");
    for (index, task) in todo_list.todo.iter().enumerate() {
        println!("Task {}: {}", index, task);
    }
    Ok(())
}

pub fn save_editor_to_json(editor: &String, home_directory: &String) -> Result<(), std::io::Error> {
    let config = Config { editor: editor.to_string() };
    let json_str = serde_json::to_string(&config)?;

    // Scrivi il JSON nel file
    let mut file = File::create(format!("{home_directory}/.pine/.config.json"))?;
    file.write_all(json_str.as_bytes())?;

    Ok(())
}

pub fn get_editor_from_json(home_directory: &String) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(format!("{home_directory}/.pine/.config.json"))?;
    let mut json_str = String::new();
    file.read_to_string(&mut json_str)?;
    let config: Config = serde_json::from_str(&json_str)?;
    Ok(config.editor)
}

pub fn get_home_directory() -> String {
    match env::var("HOME") {
        Ok(home_dir) => {
            home_dir
        },
        Err(_) => {
            // In caso di errore, restituisci una fallback (ad esempio la directory corrente)
            let current_dir = env::current_dir().unwrap();
            current_dir.to_string_lossy().to_string()
        }
    }
}

pub fn open_file_with_editor(file_path: String, editor_name: String) {
    let command = Command::new(editor_name)
        .arg(file_path)
        .spawn();

    match command {
        Ok(mut child) => {
            let status = child.wait();
        }
        Err(err) => {
            eprintln!("Errore nell'esecuzione dell'editor: {}", err);
        }
    }
}

pub fn list(dir_path: String) -> Result<Vec<PathBuf>, io::Error> {
    let entries: Result<Vec<_>, io::Error> = fs::read_dir(dir_path)?
        .map(|res| res.map(|e| e.path()))
        .collect();

    match entries {
        Ok(mut entries) => {
            entries.sort();
            Ok(entries)
        }
        Err(err) => Err(err),
    }
}
