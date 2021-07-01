mod utils;

fn warn() {
    println!("You didn\'t provide any file names!");
}

pub fn process_c_command(files: Vec<String>) {
    if files.len() == 0 {
        warn();
    } else {
        let mut first: bool = true;
        let extension: String = String::from("c");
        let contents: String = String::from("#include <stdio.h>\n\nint main(int argc, char* argv[])\
                                            {\n    printf(\"%s\", \"Hello world!\\n\");\n\n    return 0;\n}\n");
        let empty_contents: String = String::from("\n");
        for file in files {
            if first {
                utils::create_file(&file, &extension, &contents);
                first = false;
            } else {
                utils::create_file(&file, &extension, &empty_contents);
            }
        }
    }
}

pub fn process_h_command(files: Vec<String>) {
    if files.len() == 0 {
        warn();
    } else {
        let extension: String = String::from("h");

        for file in files {
            let contents: String = String::from(format!("#ifndef _{}_\n#define _{}_\n\n\n\n\n\n#endif\n",
                                                        file.to_uppercase(), file.to_uppercase()));
            utils::create_file(&file, &extension, &contents);
        }
    }
}
