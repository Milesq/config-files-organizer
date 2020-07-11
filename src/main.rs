use configuer::Configuer;
use serde::{Deserialize, Serialize};
use std::env;

mod copy_files;
mod crud_dirs;
mod messages;

use copy_files::select_to_copy;
use crud_dirs::open_crud;
use messages::*;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
struct MyData {
    dirs: Vec<String>,
}

fn main() {
    let config = Configuer::with_file("config-files-ini.bin").on_create(|| {
        let dirs = open_crud(None);
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
        return;
    }

    let files = select_to_copy(config.data.dirs);

    for file in files {
        println!("{}", file);
    }
}
