//! Ensure that roadmap edits cannot silently drop a milestone or its pentest.
use crate::Result;
use std::fs;

pub(crate) fn check() -> Result {
    let mut versions = Vec::new();
    for entry in fs::read_dir("docs/roadmap")? {
        let path = entry?.path();
        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            versions.extend(validate(&fs::read_to_string(path)?)?);
        }
    }
    versions.sort_unstable();
    if versions != (1..=110).collect::<Vec<_>>() {
        return Err("roadmap must specify each 0.1.0 through 0.110.0 exactly once".into());
    }
    Ok(())
}

fn validate(text: &str) -> Result<Vec<u32>> {
    let mut versions = Vec::new();
    for block in text.split("\n## ").skip(1) {
        let title = block.lines().next().ok_or("missing roadmap title")?;
        let version = title.split_whitespace().next().ok_or("missing version")?;
        let minor = version
            .strip_prefix("0.")
            .and_then(|v| v.strip_suffix(".0"))
            .ok_or("invalid roadmap version")?
            .parse::<u32>()?;
        for required in [
            "**Setup:**",
            "**Goal:**",
            "**Deliverables:**",
            "**Verification:**",
            "**Exit criteria:**",
            "**pentest**",
        ] {
            if !block.contains(required) {
                return Err(format!("{version} lacks {required}").into());
            }
        }
        versions.push(minor);
    }
    Ok(versions)
}

#[cfg(test)]
mod tests {
    use super::validate;

    const PASS: &str = "\n## 0.1.0 — setup\n**Setup:** baseline\n**Goal:** scope\n**Deliverables:** code\n**Verification:** tests\n**Exit criteria:** **pentest**";

    #[test]
    fn accepts_complete_milestone() {
        assert_eq!(validate(PASS).ok(), Some(vec![1]));
    }

    #[test]
    fn missing_verification_or_pentest_is_an_error() {
        for field in ["**Verification:**", "**Exit criteria:**", "**pentest**"] {
            assert!(validate(&PASS.replace(field, "")).is_err());
        }
    }

    #[test]
    fn rejects_malformed_version() {
        assert!(validate(&PASS.replace("0.1.0", "0.1.x")).is_err());
    }
}
