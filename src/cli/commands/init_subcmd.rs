use clap::{App, Arg, SubCommand};

pub fn get_init_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("init")
        .about("Initializes a new storykeep repository")
        .arg(
            Arg::with_name("config_file")
                .help("The configuration file to use")
                .index(1),
        )
}
