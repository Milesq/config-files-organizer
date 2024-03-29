use configuer::Configuer;
use serde::{Deserialize, Serialize};
use std::env;

mod copy_files;
mod crud_dirs;
mod messages;

use copy_files::{copy_file, select_to_copy};
use crud_dirs::open_crud;
use messages::*;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
struct MyData {
    dirs: Vec<String>,
}

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    let mut config = Configuer::with_file(".config-files.bin").on_create(|| {
        println!("You have to add at least one directory");

        let dirs = open_crud(Vec::new());
        MyData { dirs }
    });

    let args: Vec<_> = env::args().collect();

    let switch_is_set = |switches: &[&str]| {
        args.iter()
            .any(|el| switches.iter().any(|switch| switch == el))
    };

    if switch_is_set(&["-h", "--help"]) {
        println!("{}", HELP_MSG);
        return;
    } else if switch_is_set(&["-m", "--manage-dirs"]) {
        config.data.dirs = open_crud(config.data.dirs);
        config.save();

        return;
    }

    let files = select_to_copy(config.data.dirs);

    for file in files {
        copy_file(file);
    }
}
