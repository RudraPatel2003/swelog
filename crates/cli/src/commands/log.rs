use chrono::Local;
use clap::Args;
use config::utils::read_config_file;
use daily_log::{
    file::get_daily_log_file_name,
    write::write_daily_log_from_config,
};
use miette::Result;
use owo_colors::OwoColorize;

use crate::shared::daily_log_args::DailyLogArgs;

#[derive(Debug, Args)]
pub struct LogArgs {
    #[command(flatten)]
    daily_log_args: DailyLogArgs,
}

impl LogArgs {
    pub fn run(self) -> Result<()> {
        let swelog_config = read_config_file()?;

        let today = Local::now().date_naive();

        let log_date = self.daily_log_args.resolve_log_date(today)?;

        write_daily_log_from_config(
            &swelog_config,
            &log_date,
            self.daily_log_args.overwrite(),
            self.daily_log_args.keep_work_file(),
        )?;

        let daily_log_file_name = get_daily_log_file_name(&log_date);

        println!("Logged your work into {}", daily_log_file_name.cyan());

        Ok(())
    }
}
