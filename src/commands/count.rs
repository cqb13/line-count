use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

pub fn count_command(
    path: PathBuf,
    files: Vec<String>,
    single_directory: bool,
    detailed: bool,
) -> HashMap<OsString, Vec<i32>> {
    let mut results: HashMap<OsString, Vec<i32>> = HashMap::new();

    if path.is_file() {
        let file_ending = match path.extension() {
            Some(ending) => ending.to_owned(),
            None => panic!("Failed to get file extension"),
        };

        let lines = match scan_file(path) {
            Ok(lines) => lines,
            Err(error) => panic!("{}", error),
        };

        results.insert(file_ending, vec![lines]);

        return results;
    }

    results
}

fn scan_file(path: PathBuf) -> Result<i32, String> {
    let mut lines = 0;

    let file = File::open(path);
    let mut buf_reader = match file {
        Ok(file) => BufReader::new(file),
        Err(error) => return Err(error.to_string()),
    };
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("Failed to read file contents");

    for _ in contents.lines() {
        lines += 1;
    }

    Ok(lines)
}
