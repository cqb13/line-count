pub mod cli;
pub mod commands;
pub mod utils;

use std::collections::HashMap;
use std::ffi::OsString;

use crate::cli::{Arg, Cli, Command};
use crate::commands::count::count_command;
use crate::commands::install::install_command;
use crate::utils::{get_current_directory_path, validate_and_convert_path, OS};

fn main() {
    let cli = Cli::new()
        .with_default_command("help")
        .with_bin("lc")
        .with_commands(vec![
        Command::new("version", "Displays the current version of github-stats").with_short('v'),
        Command::new("install", "Installs the files and directories"),
        Command::new("help", "Displays the help message").with_short('h'),
        Command::new("count", "Counts lines within files in a directory")
            .with_arg(
                Arg::new()
                    .with_name("path")
                    .with_short('p')
                    .with_long("path")
                    .with_value_name("PATH")
                    .with_help("The path to a directory or file to scan"),
            )
            .with_arg(
                Arg::new()
                    .with_name("files")
                    .with_short('f')
                    .with_long("files")
                    .with_value_name("FILES")
                    .with_help("The files to count lines in (rs,py,java,etc...)"),
            )
            .with_arg(
                Arg::new()
                    .with_name("single-directory")
                    .with_short('s')
                    .with_long("single-directory")
                    .with_help("Only scans the single directory and wont scan child directories"),
            )
            .with_arg(
                Arg::new()
                    .with_name("detailed")
                    .with_short('d')
                    .with_long("detailed")
                    .with_help(
                        "Displays the amount of files of each type and the lines in each file type",
                    ),
            ),
    ]);

    let command = cli.match_commands();

    match command.name {
        "version" => cli.version(),
        "install" => {
            let os = match std::env::consts::OS {
                "windows" => OS::Windows,
                "macos" => OS::Mac,
                _ => panic!("Unsupported OS"),
            };

            install_command(&os);
        }
        "help" => cli.help(),
        "count" => {
            let path = command.get_value_of("path").to_option();
            let files = command.get_value_of("files").to_option();
            let single_directory = command.has("single-directory");
            let detailed = command.has("detailed");

            let path = match path {
                Some(path) => validate_and_convert_path(path),
                None => validate_and_convert_path(get_current_directory_path()),
            };

            let path = match path {
                Ok(path) => path,
                Err(error) => panic!("{}", error),
            };

            if !path.is_file() && files.is_none() {
                panic!("If the path is not a file you must provide file extensions")
            }

            if path.is_file() && files.is_some() {
                println!("The path you provided goes to a file, but you also provided file extensions, only the file at the path will be scanned");
            }

            let files: Vec<String> = match files {
                Some(file_list) => file_list.split(",").map(|s| s.to_string()).collect(),
                None => Vec::new(),
            };

            let count_result = count_command(path, files, single_directory);
            display_count_results(count_result, detailed)
        }
        _ => cli.help(),
    }
}

fn display_count_results(count_result: HashMap<OsString, Vec<i32>>, detailed: bool) {
    let elements: Vec<(&OsString, &Vec<i32>)> = count_result.iter().collect();

    if detailed {
        for (file_ending, lines) in elements.iter() {
            let mut total_lines = 0;

            for file_lines in lines.iter() {
                total_lines += file_lines;
            }

            println!(
                "\"{}\" files have {} lines across {} files",
                file_ending.to_string_lossy(),
                total_lines,
                lines.len()
            );
        }
    } else {
        let mut total_lines = 0;
        let mut files = 0;

        for (_, lines) in elements.iter() {
            files += lines.len();

            for file_lines in lines.iter() {
                total_lines += file_lines;
            }
        }

        println!("{} lines across {} files", total_lines, files);
    }
}
