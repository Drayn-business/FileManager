use std::{
    fs::{self},
    path::PathBuf,
};

pub fn get(path: &PathBuf) -> Option<Vec<String>> {
    if path.is_file() {
        return None;
    }

    let paths: fs::ReadDir = fs::read_dir(path).unwrap();

    let mut filenames: Vec<String> = vec!["..".to_string()];

    for path in paths {
        filenames.push(path.unwrap().file_name().into_string().unwrap());
    }

    return Some(filenames);
}
