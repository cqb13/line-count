# Line Count (lc)

`lc` is a CLI tool designed to count the lines of code in files within directories, providing various options to filter and display the results.

## Installation

`lc` can be installed either by cloning the repository and building from source or by downloading a precompiled executable from the release page. Below are the instructions for both methods:

### Installing from Source

1. **Clone the Repository**:
   Clone `lc` from GitHub to your local machine using the following command:

   ```bash
   git clone https://github.com/cqb13/line-count.git
   cd line-count
   ```

2. **Build and Install with Cargo** (requires Rust's cargo tool):
   If you have Rust and Cargo installed, you can directly build and install the application using:

   ```bash
   cargo build --release
   ./target/release/lc install
   ```

### Installing from Precompiled Executables

1. **Download the Latest Release**:
   Go to the [Releases page](https://github.com/cqb13/line-count/releases) of the `lc` repository and download the appropriate executable for your operating system.

2. **Run Install Command**:
   After downloading, navigate to the download location and run the following command to set up `lc`:

   ```bash
   ./lc install
   ```

   This command will set up `lc` on your system, making it ready for use.

### Post-Installation

After installing `lc`, you can run `lc help` to see all available commands and how to use them. Ensure that the installation path of `lc` is added to your system's PATH, so it can be run from any terminal session.

## Usage

```bash
USAGE:
    lc [COMMAND] [OPTIONS]

COMMANDS:
    version      -v
        Displays the current version of github-stats
    install      -
        Installs the files and directories
    help         -h
        Displays the help message
    count        -
        Counts lines within files in a directory
        -p           --path       <PATH>       The path to a directory or file to scan
        -f           --files      <FILES>      The files to count lines in (rs,py,java,etc...)
        -s           --single-directory <>           Only scans the single directory and wont scan child directories
        -d           --detailed   <>           Displays the amount of files of each type and the lines in each file type
```

## Contributing

Contributions are welcome! Feel free to fork this repository and submit pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
