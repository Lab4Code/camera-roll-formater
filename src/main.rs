use std::fs::{self};

use chrono::{DateTime, Datelike, Utc};
use rfd::FileDialog;
use walkdir::DirEntry;

const BASE_FOLDER_NAME: &str = "camera_roll";
fn main() {
    let path = FileDialog::new()
        .add_filter("zip_files", &["zip"])
        .set_directory("/")
        .pick_folder()
        .expect("No file selected");
    handle_folder_existance(vec![String::from(BASE_FOLDER_NAME)].as_ref());
    walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .for_each(|e| {
            if !e.path().is_dir() {
                match e.path().extension() {
                    Some(ext) => {
                        if ext == "jpg"
                            || ext == "jpeg"
                            || ext == "png"
                            || ext == "gif"
                            || ext == "mp4"
                            || ext == "mov"
                            || ext == "avi"
                            || ext == "wmv"
                            || ext == "mpeg"
                            || ext == "mkv"
                            || ext == "webm"
                        {
                            ();
                            let folder_names = get_folder_name(&e);
                            let file_name = e.file_name().to_str().unwrap();
                            let from = e.path().to_str().unwrap();
                            handle_folder_existance(&folder_names);
                            let path = format!("{}/{}", folder_names[2], file_name);
                            println!("Path: {}", path);
                            match fs::metadata(&path) {
                                Ok(_) => println!("File exists!: {}", path),
                                Err(_) => {
                                    fs::copy(from, path).expect("Failed to copy file");
                                    ()
                                }
                            }
                        } else {
                            println!("Not a media file: {:?}", e.path());
                        }
                    }
                    None => println!("No extension"),
                }
            }
        });
}

pub fn handle_folder_existance(file_paths: &Vec<String>) {
    file_paths
        .iter()
        .for_each(|file_path| match fs::metadata(file_path) {
            Ok(_) => (),
            Err(_) => {
                fs::create_dir(file_path).expect("Failed to create directory");
                ()
            }
        });
}

pub fn get_folder_name(file: &DirEntry) -> Vec<String> {
    let metadata = file.metadata();
    let created = metadata.as_ref().unwrap().created().unwrap();
    let modified = metadata.as_ref().unwrap().modified().unwrap();
    let dtc = DateTime::<Utc>::from(created);
    let dtm = DateTime::<Utc>::from(modified);
    let mut year = dtc.year();
    let mut month = dtc.month();
    if dtc > dtm {
        year = dtm.year();
        month = dtm.month();
    }
    let base_folder = String::from(BASE_FOLDER_NAME);
    return vec![
        format!("{}", base_folder),
        format!("{}/{}", base_folder, year),
        format!("{}/{}/{}", base_folder, year, month),
    ];
}
