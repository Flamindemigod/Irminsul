use log::{error, trace};
use std::{fs::metadata, path::PathBuf, time::SystemTime};


pub fn get_last_modified(target: PathBuf) -> Result<SystemTime, ()> {
    if !target.exists() {
        eprintln!("File {} does not exist", target.display());
        return Err(());
    }
    trace!("checking {} for updates", target.display());
    let tmeta = metadata(&target).map_err(|err| error!("Failed to get metadata {err}"))?;
    let tfile_mtime = tmeta
        .modified()
        .map_err(|err| error!("Failed to Read Modified Time {err}"))?;
    return Ok(tfile_mtime);
}

