use dialoguer::{Input, MultiSelect, Select};
use std::{convert::From, path::Path};

mod console_utils;

use console_utils as utils;

pub fn open_crud(mut dirs: Vec<String>) -> Vec<String> {
    loop {
        utils::clear_console();

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
                            Err("This directory doesn't exists")
                        } else {
                            Ok(())
                        }
                    })
                    .interact()
                    .unwrap();

                dirs.push(new_dir);
            }
            MenuOption::RmDir => {
                if dirs.len() == 0 {
                    println!("There are no directories");
                    continue;
                }

                let mut to_remove = MultiSelect::new()
                    .items(dirs.as_slice())
                    .interact()
                    .unwrap();

                to_remove.sort_by(|a, b| b.partial_cmp(a).unwrap());

                for to_remove_item in to_remove {
                    println!("{}", to_remove_item);
                    dirs.remove(to_remove_item);
                }
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

impl From<usize> for MenuOption {
    fn from(n: usize) -> Self {
        use MenuOption::*;

        match n {
            0 => AddDir,
            1 => ListDir,
            2 => RmDir,
            3 => Exit,
            _ => panic!(),
        }
    }
}

const ADD_DIR: &str = "Add Directory";
const LIST_DIRS: &str = "Show Directories";
const RM_DIR: &str = "Remove Directory";
const EXIT: &str = "Exit";

fn menu() -> MenuOption {
    let selected_opt = Select::new()
        .items(&[ADD_DIR, LIST_DIRS, RM_DIR, EXIT])
        .interact()
        .unwrap();

    selected_opt.into()
}
