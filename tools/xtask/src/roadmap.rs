//! Ensure that roadmap edits cannot silently drop a milestone or its pentest.
use crate::Result;
use std::fs;

pub(crate) fn check() -> Result {
    let versions = validate(&fs::read_to_string("docs/RELEASE_PLAN.md")?)?;
    let mut expected: Vec<String> = (1..=110).map(|minor| format!("0.{minor}.0")).collect();
    expected.extend(["1.0.0-rc.N".to_owned(), "1.0.0".to_owned()]);
    if versions != expected {
        return Err(
            "roadmap must specify 0.1.0 through 0.110.0, then RC and 1.0, exactly once in order"
                .into(),
        );
    }
    Ok(())
}

fn validate(text: &str) -> Result<Vec<String>> {
    let mut versions = Vec::new();
    for block in text.split("\n### ").skip(1) {
        // A following section must not supply fields missing from this milestone.
        let block = block.split("\n## ").next().ok_or("missing milestone")?;
        let title = block.lines().next().ok_or("missing roadmap title")?;
        let version = title.split_whitespace().next().ok_or("missing version")?;
        if !matches!(version, "1.0.0-rc.N" | "1.0.0") {
            let _minor = version
                .strip_prefix("0.")
                .and_then(|v| v.strip_suffix(".0"))
                .ok_or("invalid roadmap version")?
                .parse::<u32>()?;
        }
        for required in [
            "**Setup:**",
            "**Goal:**",
            "**Deliverables:**",
            "**Verification:**",
            "**Exit criteria:**",
            "pentest",
        ] {
            if !block.contains(required) {
                return Err(format!("{version} lacks {required}").into());
            }
        }
        versions.push(version.to_owned());
    }
    Ok(versions)
}

#[cfg(test)]
mod tests {
    use super::validate;

    const PASS: &str = "\n### 0.1.0 — setup\n**Setup:** baseline\n**Goal:** scope\n**Deliverables:** code\n**Verification:** tests\n**Exit criteria:** **pentest**";

    #[test]
    fn accepts_complete_milestone() {
        assert_eq!(validate(PASS).ok(), Some(vec!["0.1.0".to_owned()]));
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

    #[test]
    fn validates_grouped_milestones_and_production_gates() {
        let document = format!(
            "# Roadmap\n\n## Foundation\n{PASS}\n## Production\n{}\n{}",
            PASS.replace("0.1.0", "1.0.0-rc.N"),
            PASS.replace("0.1.0", "1.0.0")
        );
        assert_eq!(
            validate(&document).ok(),
            Some(vec![
                "0.1.0".to_owned(),
                "1.0.0-rc.N".to_owned(),
                "1.0.0".to_owned()
            ])
        );
        assert!(validate(&document.replace("**Verification:**", "")).is_err());
        let incomplete = format!(
            "{}\n## Next section\n**Verification:** tests",
            PASS.replace("**Verification:**", "")
        );
        assert!(validate(&incomplete).is_err());
    }
}
