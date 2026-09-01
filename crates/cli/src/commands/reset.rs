use clap::Args;
use config::{
    config_file::read_config_file,
    setup::swelog_paths::SwelogPaths,
    work_file::{
        create_or_reset_work_file,
        read_work_file,
    },
};
use miette::Result;
use owo_colors::OwoColorize;
use undo::snapshot::{
    UndoSnapshot,
    get_undo_snapshot_file_path,
    write_undo_snapshot,
};

#[derive(Debug, Args)]
pub struct ResetArgs {}

impl ResetArgs {
    pub fn run(self) -> Result<()> {
        let _ = self;

        let swelog_config = read_config_file()?;

        let swelog_paths = SwelogPaths::new(&swelog_config);

        let work_file_content = read_work_file(&swelog_paths)?;

        let undo_snapshot = UndoSnapshot { created_file: None, work_file_content };

        let undo_snapshot_file_path = get_undo_snapshot_file_path()?;

        write_undo_snapshot(&undo_snapshot_file_path, &undo_snapshot)?;

        create_or_reset_work_file(&swelog_config)?;

        println!("Reset work file at {}", swelog_paths.work_file.display().cyan());

        Ok(())
    }
}
