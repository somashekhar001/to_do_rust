use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task{
    pub text: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
 
      pub fn new(text: String)->Task {
        let created_at : DateTime<Utc> = Utc::now();
        Task{text , created_at}
      } 
}


pub fn collect_tasks(mut file: &File) -> Result<Vec<Task>>{
       file.seek(SeekFrom::Start(0))?;// Rewind the file before.
       let tasks  =  match serde_json:: from_reader(file){
        Ok(tasks)=>tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) =>Err(e)?,
       };
       file.seek(SeekFrom::Start(0))?;// Rewind the file after.
       Ok(tasks)
}
use std::fs::{File, OpenOptions};

pub fn add_task(journal_path: PathBuf, task: Task)->Result<()>{
       //open the file
       let mut file = OpenOptions::new().read(true).write(true).create(true).open(journal_path)?;

       //comsume the file's contents as a vector of tasks
       let mut tasks = collect_tasks(&file);

       // Write the modified task list back into the file
       tasks.push(task);
       serde_json::to_writer(file,&tasks)?;
       Ok(())
}

use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};

pub fn complete_task(journal_path: PathBuf, task: Task, task_position: usize) -> Result<()> {
    // Open the file
    let file = OpenOptions::new().read(true).write(true).open(journal_path)?;

    // Consume the file's contents as a vector of tasks.
    let tasks = collect_tasks(&file);
    //remove the task 
    if task_position == 0 || task_position > tasks.len() {
       return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }
    tasks.remove(task_position-1);

    //Rewind and truncate the file
    file.seek(SeekFrom::Start(0))?;
    file.set_len(0)?;

    //write the modified task list back into the file.
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}