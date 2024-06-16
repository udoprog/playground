use std::path::Path;

use anyhow::{Context, Result};

mod git;

fn main() -> Result<()> {
    let path = Path::new("repo-0");

    let r = match gix::open(&path) {
        Ok(r) => r,
        Err(error) if matches!(error, gix::open::Error::NotARepository { .. }) => gix::init(&path)?,
        Err(error) => return Err(error).context("Failed to open or initialize cache repository"),
    };

    let mut refspecs = Vec::new();
    refspecs.push(format!("refs/tags/v4"));

    git::sync(&r, "https://github.com/actions/upload-artifact", &refspecs)?;
    Ok(())
}
