use clap::{App, AppSettings};

mod commands;

pub mod constants;

pub fn get_cli_app<'a, 'b>() -> App<'a, 'b> {
    App::new("storybook")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Dídac S. <didac.semente@gmail.com>")
        .about("Version control tailored for writers")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommands(commands::get_commands())
}
