use std::fs::{File, self};
use std::io;
use std::io::BufReader;
use std::path::Path;
use glob::glob;

use exif::{DateTime, In, Reader, Value, Tag};

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

fn exif(file:File) {
    let exif = Reader::new().read_from_container(
        &mut BufReader::new(&file)).unwrap();
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

fn main() -> io::Result<()> {
    // Needs to be able to accept a dir
    //let file = File::open("test.JPG").unwrap();



    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_input);


    match user_input.to_lowercase().trim() {
        "r" => process_raw(),
        "j" => process_jpg(),
        _ => println!("Please choose either r or j"),
    }

    let photo_path = Path::new("/run/media/mikem/disk/DCIM/111_FUJI");


    //exif(file);
    for entry in glob("/run/media/mikem/disk/**/*").expect("Failed to read glob pattern."){
        if let Ok(path) = entry {

            //incompatible types
            exif(entry);
        }




    }
    Ok(())
}
