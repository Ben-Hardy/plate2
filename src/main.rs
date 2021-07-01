use structopt::StructOpt;

mod cli;
mod commands;

use cli::{Action::*, CommandLineArgs};
fn main() {
    let CommandLineArgs {
        action,
    } = CommandLineArgs::from_args();

    match action {
        C { names } => commands::process_c_command(names),
        H { names } => commands::process_h_command(names),
    }

}
