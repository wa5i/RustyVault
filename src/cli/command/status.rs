use clap::Parser;
use derive_more::Deref;

use crate::{errors::RvError, cli::command::{self, CommandExecutor}};

#[derive(Parser, Deref)]
#[command(
    author,
    version,
    about = r#"Prints the current state of RustyVault including whether it is sealed and if HA
mode is enabled. This command prints regardless of whether the Vault is sealed."#
)]
pub struct Status {
    #[deref]
    #[command(flatten, next_help_heading = "HTTP Options")]
    http_options: command::HttpOptions,

    #[command(flatten, next_help_heading = "Output Options")]
    output: command::OutputOptions,
}

impl CommandExecutor for Status {
    #[inline]
    fn main(&self) -> Result<(), RvError> {
        let client = self.client()?;
        let sys = client.sys();

        match sys.seal_status() {
            Ok(ret) => {
                if ret.response_status == 200 {
                    self.output.print_value(ret.response_data.as_ref().unwrap(), true)?;
                } else if ret.response_status == 204 {
                    println!("ok");
                } else {
                    ret.print_debug_info();
                }
            }
            Err(e) => eprintln!("{}", e),
        }
        Ok(())
    }
}
