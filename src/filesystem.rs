use std::fs;

pub fn get(path: &str) {
    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display());
    }
}