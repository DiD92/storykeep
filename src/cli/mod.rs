use clap::{App, AppSettings, Arg};

mod commands;

pub mod constants;

pub fn get_cli_app<'a, 'b>() -> App<'a, 'b> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("Dídac S. <didac.semente@gmail.com>")
        .about("Version control tailored for writers")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommands(commands::get_commands())
        .arg(
            Arg::with_name("dry-run")
                .long("dry-run")
                .multiple(false)
                .help("Run the commands but do not perform any changes"),
        )
}
