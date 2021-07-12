use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Create a basic "Hello World" C file. All additional files will be empty.
    C {
        filenames: Vec<String>,
    },
    /// Create a C/C++ header file with pre-made ifndef guard included. Each file's guard will match the file name.
    H {
        filenames: Vec<String>,
    },
    /// Create a matching pair of C and header files. Each C file will include their corresponding header file.
    CH {
        filenames: Vec<String>
    },
    /// Create a C++ "Hello World" file. All additional files will be empty.
    CPP {
        filenames: Vec<String>
    },
    /// Create a C++ file with matching header file. Each C++ file will include their corresponding header file.
    CPPH {
        filenames: Vec<String>
    },
    /// Create a C project with header files and a pre-populated makefile that can build, run, and clean the project.
    /// /// The project will be created in a directory named after the first included filename.
    CPROJ {
        filenames: Vec<String>
    },
    /// Create a C++ project with header files and a pre-populated makefile that can build, run, and clean the project.
    /// The project will be created in a directory named after the first included filename.
    CPPPROG {
        filenames: Vec<String>
    },
    /// Create a HTML skeleton file
    HTML {
        filenames: Vec<String>
    },
    /// Create a website in the directory named after the first included filename. A CSS, JS, and image directory are created as is an empty stylesheet.css.
    /// Each filename included will get their own HTML file.
    WEBSITE {
        filenames: Vec<String>
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
name = "Plate 2",
about = "A command-line application that creates files with pre-created templates. Basically a fancy \
         version of touch."
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
}