//! Ensure that roadmap edits cannot silently drop a milestone or its pentest.
use crate::Result;
use std::fs;

pub(crate) fn check() -> Result {
    let text = fs::read_to_string("docs/RELEASE_PLAN.md")?;
    let versions = validate(&text)?;
    let mut expected: Vec<String> = (1..=137).map(|minor| format!("0.{minor}.0")).collect();
    expected.extend(["1.0.0-rc.N".to_owned(), "1.0.0".to_owned()]);
    if versions != expected {
        return Err(
            "roadmap must specify 0.1.0 through 0.137.0, then RC and 1.0, exactly once in order"
                .into(),
        );
    }
    check_coverage(&text, &versions)?;
    Ok(())
}

fn check_coverage(text: &str, versions: &[String]) -> Result {
    let section = text
        .split_once("\n## Requirement coverage\n")
        .ok_or("missing requirement coverage matrix")?
        .1;
    let section = section
        .split("\n## ")
        .next()
        .ok_or("empty coverage section")?;
    let mut ids = Vec::new();
    for line in section.lines().filter(|line| line.starts_with("| REQ-")) {
        let cells: Vec<&str> = line.split('|').map(str::trim).collect();
        let ["", id, idea, owners, acceptance, evidence, ""] = cells.as_slice() else {
            return Err("invalid requirement coverage row".into());
        };
        if [idea, owners, acceptance, evidence]
            .iter()
            .any(|cell| cell.is_empty())
        {
            return Err(format!("{id} lacks requirement ownership or evidence status").into());
        }
        for owner in owners.split(", ") {
            if !versions.iter().any(|version| version == owner) {
                return Err(format!("{id} references unknown owner {owner}").into());
            }
        }
        ids.push(*id);
    }
    let expected: Vec<String> = (1..=26).map(|n| format!("REQ-{n:02}")).collect();
    if ids != expected {
        return Err("coverage matrix must contain REQ-01 through REQ-26 exactly once".into());
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
    use super::{check_coverage, validate};

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

    #[test]
    fn coverage_requires_all_rows_and_real_milestone_owners() {
        let mut matrix = "\n## Requirement coverage\n".to_owned();
        for n in 1..=26 {
            matrix.push_str(&format!(
                "| REQ-{n:02} | idea | 0.1.0 | negative fixture | Planned |\n"
            ));
        }
        let versions = vec!["0.1.0".to_owned()];
        assert!(check_coverage(&matrix, &versions).is_ok());
        assert!(check_coverage(&matrix.replace("0.1.0", "0.999.0"), &versions).is_err());
        assert!(check_coverage(&matrix.replace("REQ-26", "REQ-25"), &versions).is_err());
        assert!(check_coverage(&matrix.replace("| Planned |", "| |"), &versions).is_err());
        assert!(check_coverage("", &versions).is_err());
    }
}
