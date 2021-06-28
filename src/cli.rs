use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    C {
        names: Vec<String>,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(
name = "Rusty Journal",
about = "A command line to-do app written in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
}