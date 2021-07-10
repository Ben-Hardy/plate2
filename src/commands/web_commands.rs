use crate::commands::utils::{create_directory, create_file};
use std::path::Path;

pub fn process_html_command(filenames: Vec<String>) {
    let contents = String::from("<!DOCTYPE html>\n<html>\n\n    <meta charset=\"utf-8\
            \">\n\n    <head>\n        <title>Hello, World!</title>\n    </head>\n\n    <body>\n        \
            <p>Hello, World!</p>\n    </body>\n</html>\n");
    let extension = String::from("html");

    for filename in filenames {
        create_file(&filename, &extension, &contents);
    }
}