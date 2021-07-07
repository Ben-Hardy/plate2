use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::fs;

pub fn create_file(name: &String, extension: &String, contents: &String) {
    let mut filename= name.clone();

    if extension.len() > 0 {
        filename.push_str(&*format!(".{}", extension.clone()));
    }


    let file_path: &Path = Path::new(&filename);
    let display = file_path.display();

    if file_path.is_file() {
        println!("Skipping creating {} as it already exists!", display);
    } else {
        let mut file = match File::create(&file_path) {
            Err(e) => panic!("couldn't create {}: {}", display, e),
            Ok(file) => file,
        };

        match file.write_all(contents.as_bytes()) {
            Err(e) => panic!("Write failed at {} because of {}.", display, e),
            Ok(_) => println!("{} successfully created!", display),
        }
    }
}

pub fn create_directory(name: &String) {
    assert!(fs::create_dir_all(name).is_ok());
}