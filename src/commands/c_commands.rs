use crate::commands::utils::{create_directory, create_file};
use std::path::Path;

pub fn process_c_command(filenames: Vec<String>) {

    let mut first: bool = true;
    let extension: String = String::from("c");
    let contents: String = String::from("#include <stdio.h>\n\nint main(int argc, char* argv[])\
                                        {\n    printf(\"%s\", \"Hello world!\\n\");\n\n    return 0;\n}\n");
    let empty_contents: String = String::from("\n");
    for filename in filenames {
        if first {
            create_file(&filename, &extension, &contents);
            first = false;
        } else {
            create_file(&filename, &extension, &empty_contents);
        }
    }
}

pub fn process_h_command(filenames: Vec<String>) {
    for filename in filenames {
        create_single_h_file(&filename);
    }
}

fn create_single_h_file(filename: &String) {
    let h_ext: String = String::from("h");
    let h_contents: String = String::from(format!("#ifndef _{}_\n#define _{}_\n\n\n\n\n\n#endif\n",
                                                  filename.to_uppercase(), filename.to_uppercase()));
    create_file(&filename, &h_ext, &h_contents);
}

// consider this one more
pub fn process_ch_command(filenames: Vec<String>) {
    let c_ext: String = String::from("c");

    for filename in filenames {
        let c_contents: String = String::from(format!("#include \"{}.h\"\n", filename));
        create_file(&filename, &c_ext, &c_contents);
        create_single_h_file(&filename);
    }
}

pub fn process_cpp_command(filenames: Vec<String>) {
    let extension: String = String::from("cpp");
    let contents: String = String::from("#include <iostream>\n\nusing namespace std;\n\nint \
    main(int argc, char* argv[]){\n    cout << \"Hello World!\" << endl;\n\n    \
                                       return 0;\n}\n");
    let empty_contents: String = String::from("\n");
    let mut first: bool = true;

    for filename in filenames {
        if first {
            create_file(&filename, &extension, &contents);
            first = false;
        } else {
            create_file(&filename, &extension, &empty_contents);
        }
    }
}

pub fn process_cpph_command(filenames: Vec<String>) {
    let cpp_ext: String = String::from("cpp");

    for filename in filenames {
        let cpp_contents: String = String::from(format!("#include \"{}.h\"\n", filename));
        create_file(&filename, &cpp_ext, &cpp_contents);
        create_single_h_file(&filename);
    }
}

pub fn process_cproj_command(filenames: Vec<String>) {
    let path: &Path = Path::new(&*filenames[0]);
    if path.is_dir() {
        println!("The directory {} already exists!", filenames[0]);
    } else {

        // create the directory structure first!
        create_directory(&filenames[0]);

        let src_dir: String = filenames[0].clone() + "/src";
        create_directory(&src_dir);

        let header_dir = src_dir.clone() + "/headers";
        create_directory(&header_dir);

        let target_dir: String = filenames[0].clone() + "/target";
        create_directory(&target_dir);

        let mut first: bool = true;
        let c_ext: String = String::from("c");
        let h_ext: String = String::from("h");

        // populate the directory with files!
        for filename in &filenames {
            let name = src_dir.clone() + "/" + &*filename.clone();
            let header_name = header_dir.clone() + "/" + &*filename.clone();
            if first {
                first = false;

                let contents: String = String::from(format!("#include \"headers/{}.h\"\n\nint \
                 main(int argc, char* argv[]){{\n    printf(\"%s\", \"Hello world!\\n\");\n\n    \
                return 0;\n}}\n", filename));
                create_file(&name, &c_ext, &contents);

                let h_contents: String = String::from(format!("#ifndef _{}_\n#define _{}_\n\n\
                                                                  #include <stdio.h>\n\n\n\n#endif\n",
                                                              filename.to_uppercase(), filename.to_uppercase()));
                create_file(&header_name, &h_ext, &h_contents);
            } else {
                let c_contents: String = String::from(format!("#include \"headers/{}.h\"\n", filename));
                create_file(&name, &c_ext, &c_contents);

                let h_contents: String = String::from(format!("#ifndef _{}_\n#define _{}_\n\n\n\n\n\n#endif\n",
                                                              filename.to_uppercase(), filename.to_uppercase()));
                create_file(&header_name, &h_ext, &h_contents);
            }
        }

        // finally, create the makefile
        // TODO: make it add object files for each c file then link in main compilation
        let makefile_name: String = format!("{}/makefile", filenames[0]);
        let mut makefile_contents = String::from(format!("C=gcc\nCFLAGS=-Wall -pedantic\n\nexecutables=target/{}", filenames[0]));
        let run_line = String::from(&format!("\n\nrun:\n\t./target/{}",filenames[0]));
        let clean_line = String::from("\n\nclean:\n\trm -rf target/*");
        makefile_contents.push_str(&*format!("\n\n{}: \n\t$(C) $(CFLAGS) src/{}.c -o target/{}",
                                             filenames[0], filenames[0], filenames[0]));
        makefile_contents.push_str(&*run_line);
        makefile_contents.push_str(&*clean_line);
        create_file(&makefile_name, &String::from(""), &makefile_contents);
    }
}

pub fn process_cppproj_command(filenames: Vec<String>) {
    let path: &Path = Path::new(&*filenames[0]);
    if path.is_dir() {
        println!("The directory {} already exists!", filenames[0]);
    } else {

        // create the directory structure first!
        create_directory(&filenames[0]);

        let src_dir: String = filenames[0].clone() + "/src";
        create_directory(&src_dir);

        let header_dir = src_dir.clone() + "/headers";
        create_directory(&header_dir);

        let target_dir: String = filenames[0].clone() + "/target";
        create_directory(&target_dir);

        let mut first: bool = true;
        let cpp_ext: String = String::from("cpp");
        let h_ext: String = String::from("h");

        // populate the directory with files!
        for filename in &filenames {
            let name = src_dir.clone() + "/" + &*filename.clone();
            let header_name = header_dir.clone() + "/" + &*filename.clone();
            if first {
                first = false;

                let contents: String = String::from(format!("#include \"headers/{}.h\"\n\nint \
                 main(int argc, char* argv[]){{\n    std::cout << \"Hello, world!\" << std::endl;;\n\n    \
                return 0;\n}}\n", filename));
                create_file(&name, &cpp_ext, &contents);

                let h_contents: String = String::from(format!("#ifndef _{}_\n#define _{}_\n\n\
                                                                  #include <iostream>\n\n\n\n#endif\n",
                                                              filename.to_uppercase(), filename.to_uppercase()));
                create_file(&header_name, &h_ext, &h_contents);
            } else {
                let cpp_contents: String = String::from(format!("#include \"headers/{}.h\"\n", filename));
                create_file(&name, &cpp_ext, &cpp_contents);

                let h_contents: String = String::from(format!("#ifndef _{}_\n#define _{}_\n\n\n\n\n\n#endif\n",
                                                              filename.to_uppercase(), filename.to_uppercase()));
                create_file(&header_name, &h_ext, &h_contents);
            }
        }

        let makefile_name: String = format!("{}/makefile", filenames[0]);
        let mut makefile_contents = String::from(format!("CC=g++\nCFLAGS=-Wall -pedantic\n\nexecutables=target/{}", filenames[0]));
        let run_line = String::from(&format!("\n\nrun:\n\t./target/{}",filenames[0]));
        let clean_line = String::from("\n\nclean:\n\trm -rf target/*");
        makefile_contents.push_str(&*format!("\n\n{}: \n\t$(CC) $(CFLAGS) src/{}.cpp -o target/{}",
                                             filenames[0], filenames[0], filenames[0]));
        makefile_contents.push_str(&*run_line);
        makefile_contents.push_str(&*clean_line);
        create_file(&makefile_name, &String::from(""), &makefile_contents);
    }
}