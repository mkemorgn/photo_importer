use std::io;
use std::fs::{self};
use std::path::Path;

use exif::{DateTime, In, Value, Tag};

fn process_jpg() {
    println!("jpg");

}

fn process_raw() {
    println!("raw");

}

fn create_dir_structure(year: u16, month: u8, day: u8) {
    let child_dir_name = format!("{}-0{}-0{}", &year, &month, &day);
    let parent_dir_name = format!("{}", &year);

    let parent_path = Path::new(&parent_dir_name);
    let child_path = Path::new(&child_dir_name);

    let mut file_path = std::path::PathBuf::new();
    // Keep pushing directories onto PathBuf to create nested structure
    file_path.push(parent_path);
    file_path.push(child_path);

    fs::create_dir_all(file_path);

}

fn exif(path_vec:Vec<String>) -> Result<(), exif::Error> {
    for path in path_vec {
        let file = std::fs::File::open(path)?;
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader)?;

        if let Some(field) = exif.get_field(Tag::DateTime, In::PRIMARY) {
            match field.value {
                Value::Ascii(ref vec) if !vec.is_empty() => {
                    if let Ok(datetime) = DateTime::from_ascii(&vec[0]) {
                        create_dir_structure(datetime.year, datetime.month, datetime.day);
                    }
                },
                _ => {},
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {

    println!("Raw (r) or Jpeg (j)?");
    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_input);

    match user_input.to_lowercase().trim() {
        "r" => process_raw(),
        "j" => process_jpg(),
        _ => println!("Please choose either r or j"),
    }

    let photo_path = fs::read_dir("/run/media/mikem/disk/DCIM/111_FUJI").unwrap();

    let mut path_vec = vec![];
    for entry in photo_path {
        let path = entry?.path();
        let path_str = path.to_str().unwrap();
        //println!("Path: {}", path_str);
        path_vec.push(path_str.to_owned());

    }
    exif(path_vec);

    Ok(())
}
