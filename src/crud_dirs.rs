use dialoguer::{Input, MultiSelect, Select};
use std::path::Path;

pub fn open_crud(mut dirs: Vec<String>) -> Vec<String> {
    loop {
        match menu() {
            MenuOption::Exit => break,
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
    RmDir,
    Exit,
}

const ADD_DIR: &str = "Dodaj Katalog";
const RM_DIR: &str = "Usuń Katalog";
const EXIT: &str = "Wyjście";

fn menu() -> MenuOption {
    use MenuOption::*;

    let selected_opt = Select::new()
        .items(&[ADD_DIR, RM_DIR, EXIT])
        .interact()
        .unwrap();

    match selected_opt {
        0 => AddDir,
        1 => RmDir,
        2 => Exit,
        _ => panic!(),
    }
}
