use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

pub fn count_command(
    path: PathBuf,
    files: Vec<String>,
    single_directory: bool,
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
    };

    let mut paths_to_scan = vec![path];

    while let Some(path) = paths_to_scan.pop() {
        let directory_content = match fs::read_dir(&path) {
            Ok(contents) => contents,
            Err(_) => continue,
        };

        for item in directory_content {
            let item = match item {
                Ok(item) => item,
                Err(_) => continue,
            };

            if !single_directory {
                if item.path().is_dir() {
                    paths_to_scan.push(item.path());
                    continue;
                }
            }

            let path = item.path();

            let ext = match path.extension() {
                Some(ext) => ext,
                None => continue,
            };

            if item.path().is_file()
                && files.contains(
                    &ext.to_str()
                        .expect("Failed to convert os string to string")
                        .to_string(),
                )
            {
                let lines = match scan_file(item.path()) {
                    Ok(lines) => lines,
                    Err(error) => panic!("{}", error),
                };

                let file_ending = match item.path().extension() {
                    Some(ending) => ending.to_owned(),
                    None => panic!("Failed to get file extension"),
                };

                if results.contains_key(&file_ending) {
                    let current_lines = results.get_mut(&file_ending).unwrap();
                    current_lines.push(lines);
                } else {
                    results.insert(file_ending, vec![lines]);
                }
            }
        }
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
