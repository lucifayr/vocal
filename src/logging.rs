use chrono::Local;
use std::fs::{create_dir_all, metadata, read_dir, remove_file, File};

pub fn create_log_file(dir: &str, prefix: &str) -> Result<File, std::io::Error> {
    clean_log_dir(dir)?;

    let date_str = Local::now().format("%Y-%m-%d_%H:%M:%S");
    match File::create(format!("{dir}/{prefix}_{date_str}.log",)) {
        Ok(file) => Ok(file),
        Err(_) => {
            create_dir_all(dir)?;
            File::create(format!("{dir}/{prefix}_{date_str}.log"))
        }
    }
}

pub fn clean_log_dir(dir: &str) -> Result<(), std::io::Error> {
    let entries = read_dir(dir)?;

    let mut files = Vec::new();
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let metadata = metadata(&path)?;
            let modified_time = metadata.modified()?;
            files.push((path, modified_time));
        }
    }

    if files.len() >= 10 {
        files.sort_by(|a, b| b.1.cmp(&a.1));

        for file in files.iter().skip(9) {
            let del_path = &file.0;
            remove_file(del_path)?;
        }
    }

    Ok(())
}
