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
    // Create a matching pair of C and header files. Each C file will include their corresponding header file.
    CH {
        filenames: Vec<String>
    },
    CPP {
        filenames: Vec<String>
    },
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