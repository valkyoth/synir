//! Online review reports drift; it never installs or upgrades tools.
use crate::{Result, output};
use std::fs;

pub(crate) fn freshness() -> Result {
    let stable = fetch("https://static.rust-lang.org/dist/channel-rust-stable.toml")?;
    let version = section_version(&stable, "[pkg.rust]").ok_or("missing stable Rust version")?;
    if !version.starts_with("1.98.1 ") {
        return Err(format!("review newer Rust stable: {version}").into());
    }
    for (repo, prefix, expected) in [
        ("EmbarkStudios/cargo-deny", "", "0.20.2"),
        ("rustsec/rustsec", "cargo-audit/v", "0.22.2"),
        ("actions/checkout", "v", "7.0.1"),
    ] {
        // The RustSec repository has several products, so /releases/latest is insufficient.
        let tags = output(
            "git",
            &[
                "ls-remote",
                "--tags",
                &format!("https://github.com/{repo}.git"),
            ],
        )?;
        let latest = latest_tag(&tags, prefix).ok_or("no stable tags found")?;
        if latest != expected {
            return Err(
                format!("{repo}: pinned {expected}, latest {latest}; review required").into(),
            );
        }
        println!("{repo}: {latest}");
    }
    let pin = fs::read_to_string("rust-toolchain.toml")?;
    if !pin.contains("channel = \"1.98.1\"") {
        return Err("toolchain pin drift".into());
    }
    println!("Rust: {version}; external workspace crates: forbidden (cargo xtask policy)");
    Ok(())
}

fn fetch(url: &str) -> Result<String> {
    output(
        "curl",
        &[
            "--fail",
            "--silent",
            "--show-error",
            "--max-time",
            "60",
            url,
        ],
    )
}

fn section_version<'a>(text: &'a str, section: &str) -> Option<&'a str> {
    let (_, body) = text.split_once(section)?;
    body.lines()
        .skip(1)
        .take_while(|line| !line.starts_with('['))
        .find_map(|line| line.strip_prefix("version = \"")?.strip_suffix('"'))
}

fn latest_tag(text: &str, prefix: &str) -> Option<String> {
    text.lines()
        .filter_map(|line| {
            let (_, tag) = line.split_once("refs/tags/")?;
            let version = tag.strip_prefix(prefix)?;
            let parts: Vec<u64> = version
                .split('.')
                .map(str::parse)
                .collect::<std::result::Result<_, _>>()
                .ok()?;
            if parts.len() == 3 {
                Some((parts, version))
            } else {
                None
            }
        })
        .max_by(|a, b| a.0.cmp(&b.0))
        .map(|(_, version)| version.to_owned())
}

#[cfg(test)]
mod tests {
    use super::{latest_tag, section_version};

    #[test]
    fn numeric_versions_ignore_prereleases_and_other_products() {
        let tags = "a refs/tags/cargo-audit/v0.9.0\nb refs/tags/cargo-audit/v0.22.2\nc refs/tags/platforms/v99.0.0\nd refs/tags/cargo-audit/v1.0.0-rc.1\ne refs/tags/cargo-audit/v0.22.2^{}";
        assert_eq!(latest_tag(tags, "cargo-audit/v").as_deref(), Some("0.22.2"));
        assert_eq!(latest_tag("malformed", "v"), None);
    }

    #[test]
    fn rust_version_is_scoped_to_package_section() {
        assert_eq!(
            section_version(
                "[pkg.rust]\nversion = \"1.98.1 (hash)\"\n[next]\nversion = \"2\"",
                "[pkg.rust]"
            ),
            Some("1.98.1 (hash)")
        );
        assert_eq!(
            section_version("[pkg.rust]\n[next]\nversion = \"2\"", "[pkg.rust]"),
            None
        );
    }
}
