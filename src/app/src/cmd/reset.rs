use clap::{App, ArgMatches};

use huber_common::config::Config;
use huber_common::result::Result;

use crate::cmd::CommandTrait;
use crate::service::update::{UpdateService, UpdateTrait};
use huber_common::di::di_container;

pub(crate) const CMD_NAME: &str = "reset";

pub(crate) struct ResetCmd;

impl ResetCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for ResetCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME).about("Resets huber (ex: remove installed packages)")
    }

    fn run(&self, _config: &Config, _matches: &ArgMatches<'a>) -> Result<()> {
        let container = di_container();
        let update_service = container.get::<UpdateService>().unwrap();

        println!(
            "Resetting huber by removing created caches, downloaded files and installed packages"
        );
        update_service.reset()?;
        println!("Done");

        Ok(())
    }
}