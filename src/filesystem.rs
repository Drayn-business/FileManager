use std::fs;

pub fn get(path: &str) -> Vec<String> {
    let paths = fs::read_dir(path).unwrap();

    let mut filenames: Vec<String> = vec![];

    for path in paths {
        filenames.push(path.unwrap().file_name().into_string().unwrap());
    }

    return filenames;
}