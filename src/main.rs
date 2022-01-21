mod cli;
mod file_system;
mod task;

use std::path::PathBuf;

use anyhow::anyhow;
use cli::{Action::*, CommandLineArgs};
use file_system::*;
use structopt::StructOpt;
use task::Task;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to find journal file"))?;

    match action {
        Add { text } => add_task(journal_file, Task::new(text)),
        Done { task_position } => done_task(journal_file, task_position),
        List => list_tasks(journal_file),
    }?;

    Ok(())
}
