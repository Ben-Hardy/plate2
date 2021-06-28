use structopt::StructOpt;

mod utils;
mod cli;
use cli::{Action::*, CommandLineArgs};
fn main() {
    let CommandLineArgs {
        action,
    } = CommandLineArgs::from_args();

    match action {
        C { names } => utils::process_c_command(names),
    }

}
