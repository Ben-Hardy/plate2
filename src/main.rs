use structopt::StructOpt;

mod cli;
mod commands;

use cli::{Action::*, CommandLineArgs};
fn main() {
    let CommandLineArgs {
        action,
    } = CommandLineArgs::from_args();

    match action {
        C        { filenames } => commands::process_c_command(filenames),
        H        { filenames } => commands::process_h_command(filenames),
        CH       { filenames } => commands::process_ch_command(filenames),
        CPP      { filenames } => commands::process_cpp_command(filenames),
        CPPH     { filenames } => commands::process_cpph_command(filenames),
        CPROJ   { filenames } => commands::process_cproj_command(filenames),
        CPPPROG { filenames } => commands::process_cppproj_command(filenames),
        HTML     { filenames } => commands::process_html_command(filenames),
        WEBSITE  { filenames } => commands::process_website_command(filenames),
    }
}
