//! Negative tests for repository controls using disposable first-party fixtures.
use super::{links, walk};
use crate::Result;
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT: AtomicU64 = AtomicU64::new(0);

struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Result<Self> {
        let path = std::env::temp_dir().join(format!(
            "synir-policy-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        // Refuse collisions; never take ownership of an existing directory.
        fs::create_dir(&path)?;
        Ok(Self(path))
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn source_limit_includes_tests_and_generated_files() -> Result {
    let fixture = Fixture::new()?;
    let path = fixture.0.join("generated.rs");
    fs::write(&path, "// line\n".repeat(500))?;
    assert!(walk(&fixture.0).is_ok());
    fs::write(&path, "// line\n".repeat(501))?;
    assert!(walk(&fixture.0).is_err());
    Ok(())
}

#[test]
fn implicit_build_script_is_rejected() -> Result {
    let fixture = Fixture::new()?;
    fs::write(fixture.0.join("build.rs"), "fn main() {}")?;
    assert!(walk(&fixture.0).is_err());
    Ok(())
}

#[test]
fn local_links_require_files_but_external_links_are_not_fetched() -> Result {
    let fixture = Fixture::new()?;
    let path = fixture.0.join("README.md");
    fs::write(&path, "[missing](missing.md)")?;
    assert!(links(&path).is_err());
    fs::write(fixture.0.join("missing.md"), "# Present")?;
    assert!(links(&path).is_ok());
    fs::write(&path, "[web](https://example.invalid/) [anchor](#section)")?;
    assert!(links(&path).is_ok());
    Ok(())
}
