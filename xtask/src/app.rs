use crate::command;
use clap::Parser;

#[derive(Debug, clap::Parser)]
#[command(disable_help_subcommand(true))]
pub struct Args {
    #[clap(subcommand)]
    pub subcommand: command::XTaskCommand,
}

impl<I, T> From<I> for Args
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    #[inline]
    fn from(value: I) -> Self {
        Self::parse_from(value)
    }
}

pub fn run(args: impl Into<Args>) -> Result<(), anyhow::Error> {
    let args = args.into();

    match args.subcommand {
        command::XTaskCommand::Codegen(subcommand) => match subcommand {
            command::CodeGenCommand::All => {
                command::codegen_grammar::run()?;
                command::codegen_jsonschema::run()?
            }
            command::CodeGenCommand::Grammar => command::codegen_grammar::run()?,
            command::CodeGenCommand::Jsonschema => command::codegen_jsonschema::run()?,
        },
        command::XTaskCommand::SetVersion => {
            command::set_version::run(&xshell::Shell::new().unwrap())?
        }
        command::XTaskCommand::TomlTest => command::toml_test::run(&xshell::Shell::new().unwrap())?,
        command::XTaskCommand::Dist => command::dist::run(&xshell::Shell::new().unwrap())?,
    }
    Ok(())
}
