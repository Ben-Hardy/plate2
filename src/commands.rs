mod utils;

fn warn() {
    println!("You didn\'t provide any file names!");
}

pub fn process_c_command(filenames: Vec<String>) {
    if filenames.len() == 0 {
        warn();
    } else {
        let mut first: bool = true;
        let extension: String = String::from("c");
        let contents: String = String::from("#include <stdio.h>\n\nint main(int argc, char* argv[])\
                                            {\n    printf(\"%s\", \"Hello world!\\n\");\n\n    return 0;\n}\n");
        let empty_contents: String = String::from("\n");
        for filename in filenames {
            if first {
                utils::create_file(&filename, &extension, &contents);
                first = false;
            } else {
                utils::create_file(&filename, &extension, &empty_contents);
            }
        }
    }
}

pub fn process_h_command(filenames: Vec<String>) {
    if filenames.len() == 0 {
        warn();
    } else {
        for filename in filenames {
            create_single_h_file(&filename);
        }
    }
}

fn create_single_h_file(filename: &String) {
    let h_ext: String = String::from("h");
    let h_contents: String = String::from(format!("#ifndef _{}_\n#define _{}_\n\n\n\n\n\n#endif\n",
                                                  filename.to_uppercase(), filename.to_uppercase()));
    utils::create_file(&filename, &h_ext, &h_contents);
}

// consider this one more
pub fn process_ch_command(filenames: Vec<String>) {
    if filenames.len() == 0 {
        warn();
    } else {
        let c_ext: String = String::from("c");

        for filename in filenames {
            let c_contents: String = String::from(format!("#include \"{}.h\"\n", filename));
            utils::create_file(&filename, &c_ext, &c_contents);
            create_single_h_file(&filename);
        }
    }
}

pub fn process_cpp_command(filenames: Vec<String>) {
    if filenames.len() == 0 {
        warn();
    } else {
        let extension: String = String::from("cpp");
        let contents: String = String::from("#include <iostream>\n\nusing namespace std;\n\nint \
        main(int argc, char* argv[]){\n    cout << \"Hello World!\" << endl;\n\n    \
                                           return 0;\n}\n");
        let empty_contents: String = String::from("\n");
        let mut first: bool = true;

        for filename in filenames {
            if first {
                utils::create_file(&filename, &extension, &contents);
                first = false;
            } else {
                utils::create_file(&filename, &extension, &empty_contents);
            }
        }
    }
}