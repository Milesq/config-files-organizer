use configuer::Configuer;
use serde::{Deserialize, Serialize};
use std::env;

mod messages;
use messages::*;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
struct MyData {
    dirs: Vec<String>,
}

fn main() {
    let config = Configuer::with_file("myIniFileName").on_create(|| MyData {
        dirs: vec!["Default user name".into()],
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
    copy_files(files);
}

fn copy_files(_: Vec<String>) {}

fn select_to_copy(_: Vec<String>) -> Vec<String> {
    vec![]
}
