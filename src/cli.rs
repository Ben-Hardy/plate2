use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    C {
        names: Vec<String>,
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