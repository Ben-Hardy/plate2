mod utils;
pub fn process_c_command(files: Vec<String>) {
    let extension: String = String::from("c");
    let contents: String = String::from("hello world!\n");
    utils::create_file(&files[0].clone(), &extension, &contents);
}