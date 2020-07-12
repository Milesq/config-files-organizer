use dialoguer::MultiSelect;
use std::{
    env,
    fs::{self, DirEntry},
    io,
    path::Path,
};

fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(entry);
            }
        }
    }
    Ok(())
}

pub fn select_to_copy(dirs: Vec<String>) -> Vec<String> {
    let mut files = Vec::new();

    for dir in dirs {
        visit_dirs(Path::new(&dir), &mut |entry| files.push(entry)).unwrap();
    }

    let filesx = files
        .iter()
        .map(|entry| {
            let file_name = entry.file_name();
            let path = entry.path();
            let parent_name: Option<&str> = path
                .parent()
                .map(|parent| parent.file_name().unwrap().to_str().unwrap());

            format!(
                "{}\t\t({})",
                file_name.to_str().unwrap(),
                if let Some(parent_name) = parent_name {
                    parent_name
                } else {
                    "./"
                },
            )
        })
        .collect::<Vec<_>>();

    MultiSelect::new()
        .items(filesx.as_slice())
        .interact()
        .unwrap()
        .iter()
        .map(|n| files[*n].path())
        .map(|n| n.to_str().unwrap().to_string())
        .collect()
}

pub fn copy_file(path: String) {
    let file_name = Path::new(&path).file_name().unwrap().to_str().unwrap();

    let mut current_dir = env::current_dir().unwrap();
    current_dir.push(file_name);

    if let Err(err) = fs::copy(&path, current_dir) {
        println!("Nie można skopiować pliku\n{}\n\n\n{:?}", path, err);
    }
}
