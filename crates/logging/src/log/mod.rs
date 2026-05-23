use miette::Result;

pub fn log_daily_work(overwrite_existing_daily_log: bool, save_work_file: bool) -> Result<()> {
    let swelog_config = config::read_config_file()?;

    Ok(())
}
