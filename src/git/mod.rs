mod progress;

use std::num::NonZeroU32;
use std::sync::atomic::AtomicBool;

use anyhow::{Context, Result};
use bstr::BStr;
use gix::remote::fetch::Source;
use gix::Repository;

use self::progress::Logger;

pub(crate) fn sync(repo: &Repository, url: &str, refspecs: &[String]) -> Result<()> {
    let refspecs = refspecs.iter().map(|s| BStr::new(s.as_bytes()));

    let remote = repo
        .find_fetch_remote(Some(BStr::new(url)))
        .context("Failed to find or make fetch remote")?
        .with_refspecs(refspecs, gix::remote::Direction::Fetch)?;

    let options = gix::remote::ref_map::Options::default();

    let mut progress = Logger::new();

    let shallow = NonZeroU32::new(1)
        .map(gix::remote::fetch::Shallow::DepthAtRemote)
        .unwrap_or(gix::remote::fetch::Shallow::NoChange);

    let should_interrupt = AtomicBool::new(false);

    let connect = remote.connect(gix::remote::Direction::Fetch)?;

    let outcome = connect
        .prepare_fetch(&mut progress, options)?
        .with_shallow(shallow)
        .receive(&mut progress, &should_interrupt)?;

    for mapping in &outcome.ref_map.mappings {
        let Source::Ref(r) = &mapping.remote else {
            continue;
        };

        dbg!(r);
    }

    Ok(())
}
