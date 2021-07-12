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

pub fn process_website_command(filenames: Vec<String>) {
    let path: &Path = Path::new(&*filenames[0]);
    if path.is_dir() {
        println!("The directory {} already exists!", filenames[0]);
    } else {
        // base directory
        create_directory(&filenames[0]);

        let css_dir: String = filenames[0].clone() + "/css";
        create_directory(&css_dir);

        let img_dir: String = filenames[0].clone() + "/img";
        create_directory(&img_dir);

        let js_dir: String = filenames[0].clone() + "/js";
        create_directory(&js_dir);

        let html_contents = String::from("<!DOCTYPE html>\n<html>\n\n    <meta charset=\"utf-8\
            \">\n\n    <head>\n        <title>Hello, World!</title>\n        <link href=\"css/stylesheet.css\" rel=\"stylesheet\" \
            type=\"text/css\"/>\n    </head>\n\n    <body>\n        \
            <p>Hello, World!</p>\n    </body>\n</html>\n");

        let empty_contents: String = String::from("");

        let index_filename: String = String::from(format!("{}/index", filenames[0]));

        let html_ext: String = String::from("html");
        let css_ext: String = String::from("css");

        create_file(&index_filename, &html_ext, &html_contents);

        let css_filename: String = String::from(format!("{}/css/stylesheet", filenames[0]));
        create_file(&css_filename, &css_ext, &empty_contents);

        for filename in &filenames {
            let cur_filename: String = String::from(format!("{}/{}", filenames[0], filename));
            create_file(&cur_filename, &html_ext, &html_ext);
        }
    }
}