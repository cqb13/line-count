pub mod cli;
pub mod utils;

use crate::cli::{Arg, Cli, Command};
use crate::utils::{install, validate_and_convert_path, OS};

fn main() {
    let cli = Cli::new().with_default_command("help").with_commands(vec![
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
                    .with_help("The files to count lines in (.rs,.py,.java,etc...)"),
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

            install(&os);
        }
        "help" => cli.help(),
        "count" => {
            /*
            x lines in y files

            x ".rs" files
                y lines
            z ".py" files
                q lines
             */
        }
        _ => cli.help(),
    }
}
