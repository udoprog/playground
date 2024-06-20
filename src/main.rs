use std::path::Path;

use anyhow::{Context, Result};
use bstr::BString;

mod git;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let path = Path::new("repo-0");

    let r = match gix::open(&path) {
        Ok(r) => r,
        Err(error) if matches!(error, gix::open::Error::NotARepository { .. }) => {
            gix::init_bare(&path)?
        }
        Err(error) => return Err(error).context("Failed to open or initialize cache repository"),
    };

    let mut refspecs = Vec::new();
    refspecs.push(BString::from(format!("refs/tags/v1")));
    refspecs.push(BString::from(format!("refs/heads/v1")));
    git::sync(
        &r,
        "https://github.com/taiki-e/setup-cross-toolchain-action",
        &refspecs,
    )?;
    Ok(())
}
