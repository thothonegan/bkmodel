
use clap::Clap;

#[derive(Clap)]
#[clap(version="1.0", author="Kenneth Perry <thothonegan@gmail.com>")]
pub struct CommandLineOptions {

    #[clap(short, long)]
    pub filename: String,

    #[clap(subcommand)]
    pub sub_command: SubCommand
}

#[derive(Clap)]
pub enum SubCommand {
    #[clap()]
    Inspect(Inspect),

    #[clap()]
    ViewDisplayList(ViewDisplayList),

    #[clap()]
    ViewVertexStore(ViewVertexStore),
}

// Subcommand with inspects the file
#[derive(Clap)]
pub struct Inspect
{}

#[derive(Clap)]
pub struct ViewDisplayList
{}

#[derive(Clap)]
pub struct ViewVertexStore
{}