use miette::Result;

pub fn log_daily_work(overwrite_existing_daily_log: bool, save_work_file: bool) -> Result<()> {
    let swelog_config = config::utils::read_config_file()?;

    println!("swelog_config: {:?}", swelog_config);
    println!("overwrite_existing_daily_log: {:?}", overwrite_existing_daily_log);
    println!("save_work_file: {:?}", save_work_file);

    Ok(())
}
