use clap::{App, SubCommand};

pub fn get_init_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("init").about("Initializes a new storykeep keep")
}
