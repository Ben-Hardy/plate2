use structopt::StructOpt;

mod cli;
mod commands;

use cli::{Action::*, CommandLineArgs};
fn main() {
    let CommandLineArgs {
        action,
    } = CommandLineArgs::from_args();

    match action {
        C { filenames } => commands::process_c_command(filenames),
        H { filenames } => commands::process_h_command(filenames),
        CH { filenames} => commands::process_ch_command(filenames),
    }

}
