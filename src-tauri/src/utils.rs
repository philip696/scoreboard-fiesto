use dirs;
use std::fs::File;
use std::io::{self, Error, Read};
use std::path::{Path, PathBuf};

pub fn ensure_file_exists(file_path: &Path) -> Result<(), Error> {
    if !file_path.exists() {
        match File::create(file_path) {
            Ok(_) => println!("File created: {}", file_path.display()),
            Err(e) => println!("Error creating file: {}", e),
        }
        println!("File created: {}", file_path.display());
    } else {
        println!("File already exists: {}", file_path.display());
    }
    Ok(())
}

pub fn read_file(file_path: &Path) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn get_absolute_path_in_home(relative_path: &str) -> Result<PathBuf, String> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let absolute_path = home_dir.join(relative_path);
    Ok(absolute_path)
}
