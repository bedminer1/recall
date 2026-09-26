//! `recall subject add <subject>` and `recall add <subject> <file>`.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::model::*;
use crate::store::*;

pub(crate) fn subject_add(raw_subject: &str) -> AppResult<()> {
    let root = env::current_dir().map_err(io_error)?;
    let subject = validate_slug(raw_subject, "subject")?;
    fs::write(root.join(".recall"), "Recall project\n").map_err(io_error)?;
    let base = root.join("subjects").join(&subject);
    for directory in ["resources", "quizzes", "attempts"] {
        fs::create_dir_all(base.join(directory)).map_err(io_error)?;
    }
    let progress = base.join("progress.md");
    if !progress.exists() {
        fs::write(
            &progress,
            "# Progress\n\n| date | attempt | question | difficulty | result | tries |\n|---|---|---|---|---|---|\n",
        )
        .map_err(io_error)?;
    }
    println!(
        "Added subject `{subject}` at {}",
        display_path(&base, &root)
    );
    Ok(())
}

pub(crate) fn move_resource(
    root: &Path,
    raw_subject: &str,
    raw_file: &str,
    copy: bool,
) -> AppResult<()> {
    let subject = validate_slug(raw_subject, "subject")?;
    let subject_dir = root.join("subjects").join(&subject);
    if !subject_dir.is_dir() {
        return Err(format!(
            "subject `{subject}` does not exist; run `recall subject add {subject}`"
        ));
    }
    let source = PathBuf::from(raw_file);
    if !source.is_file() {
        return Err(format!("resource file not found: {}", source.display()));
    }
    let file_name = source
        .file_name()
        .ok_or_else(|| "resource path has no file name".to_string())?;
    let destination = subject_dir.join("resources").join(file_name);
    if destination.exists() {
        return Err(format!(
            "resource already exists: {}",
            destination.display()
        ));
    }
    if copy {
        fs::copy(&source, &destination).map_err(io_error)?;
        println!(
            "Added resource (copied): {}",
            display_path(&destination, root)
        );
        return Ok(());
    }
    match fs::rename(&source, &destination) {
        Ok(()) => {}
        // A cross-device rename cannot succeed on its own; fall back to a
        // copy plus removal so the source is still moved, never left behind.
        Err(error) if error.raw_os_error() == Some(18) => {
            fs::copy(&source, &destination).map_err(io_error)?;
            fs::remove_file(&source).map_err(io_error)?;
        }
        Err(error) => return Err(io_error(error)),
    }
    println!(
        "Added resource (moved): {}",
        display_path(&destination, root)
    );
    Ok(())
}
