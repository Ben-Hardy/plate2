#Plate 2

#### A command-line application that creates files with pre-created templates. Basically a fancy version of touch.

USAGE:
plate2 [SUBCOMMAND] [FILENAME]

FLAGS:
* -h, --help:       Prints help information
* -V, --version:    Prints version information

SUBCOMMANDS:
* c:           Create a basic "Hello World" C file. All additional files will be empty
* ch:          Create a matching pair of C and header files. Each C file will include their corresponding header file

* cpp:        Create a C++ "Hello World" file. All additional files will be empty
* cpph:       Create a C++ file with matching header file. Each C++ file will include their corresponding header file
* cppprog:    Create a C++ project with header files and a pre-populated makefile that can build, run, and clean the project. The project will be created in a directory named after the first included filename

* cproj:      Create a C project with header files and a pre-populated makefile that can build, run, and clean the project. The project will be created in a directory named after the first included filename

* h:          Create a C/C++ header file with pre-made ifndef guard included. Each file's guard will match the file name

* help:       Prints this message or the help of the given subcommand(s)

* html:       Create a HTML skeleton file
* website:    Create a website in the directory named after the first included filename. A CSS, JS, and image directory are created as is an empty stylesheet.css. Each filename included will get their own HTML file

Example usage:

* `plate2 cpph testing`: creates a c++ file called testing.cpp and a header file called testing.h

* `plate2 cproj hello_world`: creates a C-based project in a directory named `hello_world`