use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::path::PathBuf;

use crate::task::Task;

fn get_tasks(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?;
    let tasks: Vec<Task> = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?;
    Ok(tasks)
}

pub fn add_task(path: PathBuf, task: Task) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    let mut tasks = get_tasks(&file)?;

    tasks.push(task);

    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn done_task(path: PathBuf, task_position: usize) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    let mut tasks = get_tasks(&file)?;

    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }
    tasks.remove(task_position);

    file.set_len(0)?;

    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn list_tasks(path: PathBuf) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    let tasks = get_tasks(&file)?;

    if tasks.len() == 0 {
        println!("Yo do not have tasks")
    } else {
        let mut index: usize = 1;
        for task in tasks {
            println!("{}: {}", index, task);
            index += 1;
        }
    }

    Ok(())
}
