use dialoguer::{Input, MultiSelect, Select};
use std::path::Path;

mod console_utils;

use console_utils as utils;

pub fn open_crud(mut dirs: Vec<String>) -> Vec<String> {
    loop {
        match menu() {
            MenuOption::Exit => break,
            MenuOption::ListDir => {
                for dir in &dirs {
                    println!("-\t{}", dir);
                }

                utils::get_char().unwrap();
            }
            MenuOption::AddDir => {
                let new_dir: String = Input::new()
                    .with_prompt("Podaj ścieżkę katalogu")
                    .validate_with(|input: &str| -> Result<(), &str> {
                        if !Path::new(input).is_dir() {
                            Err("Podana ścieżka nie jest katalogiem")
                        } else {
                            Ok(())
                        }
                    })
                    .interact()
                    .unwrap();

                dirs.push(new_dir);
            }
            MenuOption::RmDir => {
                println!("{:?}", dirs.as_slice());
            }
        }
    }

    dirs
}

enum MenuOption {
    AddDir,
    ListDir,
    RmDir,
    Exit,
}

const ADD_DIR: &str = "Dodaj Katalog";
const LIST_DIRS: &str = "Pokaż zapisane katalogi";
const RM_DIR: &str = "Usuń Katalog";
const EXIT: &str = "Wyjście";

fn menu() -> MenuOption {
    use MenuOption::*;

    let selected_opt = Select::new()
        .items(&[ADD_DIR, LIST_DIRS, RM_DIR, EXIT])
        .interact()
        .unwrap();

    match selected_opt {
        0 => AddDir,
        1 => ListDir,
        2 => RmDir,
        3 => Exit,
        _ => panic!(),
    }
}
