use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum OS {
    Windows,
    Mac,
}

impl OS {
    pub fn get_name(&self) -> &str {
        match self {
            OS::Windows => "Windows",
            OS::Mac => "Mac",
        }
    }
}

pub fn validate_and_convert_path(path: String) -> Result<PathBuf, String> {
    let real_path = Path::new(&path);

    if !real_path.exists() {
        return Err("The path does not exist.".to_owned());
    }

    Ok(real_path.to_path_buf())
}

pub fn get_current_directory_path() -> String {
    let current_dir_path = match env::current_dir() {
        Ok(path) => path,
        Err(_) => panic!("Could not get current directory path"),
    };

    current_dir_path.to_str().unwrap().to_string()
}

pub fn get_user_home_dir(os: &OS) -> String {
    let user_dir = match os {
        OS::Windows => "USERPROFILE",
        OS::Mac => "HOME",
    };
    std::env::var(format!("{}", user_dir)).unwrap()
}
