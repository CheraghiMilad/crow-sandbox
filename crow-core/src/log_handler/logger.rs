use std::fs::File;
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::Local;
use fern::Dispatch;
use log::LevelFilter;

/// Initializes the logging system by creating the log directory if necessary,
/// building the log file path, opening the log file, and configuring fern
/// to write to both stdout and the file.
pub fn init_log_handler(log_filename: &str) -> Result<()> {

    // Ensure the "logs" directory exists or create it
    let log_dir = "logs";
    ensure_log_dir_exists(log_dir)?;

    // Construct the full path to the log file
    let log_path = build_log_path(log_dir, log_filename);

    // Open the log file (creating or appending as needed)
    let file = open_log_file(&log_path)?;

    // Configure and apply the fern logger with our file and stdout
    setup_logger(file)?;
    Ok(())
}


/// Ensures that the given directory exists by creating it and any missing parents.
fn ensure_log_dir_exists(dir: &str) -> Result<()> {
    fs::create_dir_all(dir).with_context(|| format!("Failed to create log directory '{}'", dir))?;
    Ok(())
}


/// Combines a directory and filename into a full `PathBuf`.
fn build_log_path(dir: &str, filename: &str) -> PathBuf {
    Path::new(dir).join(filename)
}


/// Opens the specified log file for writing and appending, creating it if it does not exist.
fn open_log_file(path: &Path) -> Result<File> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("Failed to open log file '{}'", path.display()))
}

/// Configures the fern logger to format each message with a timestamp, target, level, and message,
/// and then chains both stdout and the provided file as output sinks.
fn setup_logger(log_file: File) -> Result<()> {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(log_file)
        .apply()
        .context("Failed to initialize fern logger")
}
