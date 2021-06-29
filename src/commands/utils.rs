use std::fs::File;
use std::path::Path;
use std::io::Write;


pub fn create_file(name: &String, extension: &String, contents: &String) {
    let filename: &str = &*format!("{}.{}", name, extension);
    let file_path: &Path = Path::new(filename);
    let display = file_path.display();

    let mut file = match File::create(&file_path) {
        Err(e) => panic!("couldn't create {}: {}", display, e),
        Ok(file) => file,
    };

    match file.write_all(contents.as_bytes()) {
        Err(e) => panic!("Write failed at {} because of {}.", display, e),
        Ok(_) => println!("{} successfully created!", display),
    }
}