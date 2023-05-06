use rand::distributions::Alphanumeric;
use rand::Rng;
use std::fs;
use std::fs::File;
use std::io::Result;
use std::io::Write;

static DATA_FOLDER: &str = "./data";

fn get_data_file_path(file_name: &str) -> String {
    format!("{DATA_FOLDER}/{file_name}")
}

fn generate_temp_file_path() -> String {
    let temp_file_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    format!("{DATA_FOLDER}/{temp_file_name}")
}

fn save_data(path: String, data: &[u8]) -> Result<()> {
    let temp_file_path = &generate_temp_file_path();
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(temp_file_path)?;

    file.write_all(data)?;

    file.sync_all()?;

    fs::rename(temp_file_path, path)
}

fn create_log(path: String) -> Result<File> {
    fs::OpenOptions::new().create(true).append(true).open(path)
}

fn append_log(mut file: &File, data: &[u8]) -> Result<()> {
    file.write_all(data)?;
    file.sync_all()
}

fn test_save_data() -> Result<()> {
    save_data(get_data_file_path("file.txt"), b"Hello world")
}

fn test_append() -> Result<()> {
    let file = create_log(get_data_file_path("x.log"))?;
    append_log(&file, b"Hello world!\n")
}

fn main() {
    match test_save_data() {
        Ok(()) => println!("Success"),
        Err(err) => {
            eprintln!("{err}")
        }
    };

    match test_append() {
        Ok(()) => println!("Success"),
        Err(err) => {
            eprintln!("{err}")
        }
    };
}
