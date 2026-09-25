//! Repository shape, source size, dependency and documentation checks.
use crate::{Result, output};
use std::{fs, path::Path};

const PACKAGES: [(&str, &str); 5] = [
    ("synir", "crates/synir"),
    ("synir-core", "crates/synir-core"),
    ("synir-host", "crates/synir-host"),
    ("synir-macros", "crates/synir-macros"),
    ("synir-xtask", "tools/xtask"),
];

pub(crate) fn check() -> Result {
    let lock = fs::read_to_string("Cargo.lock")?;
    check_lock(&lock)?;
    // --target all includes target-specific dependencies; --edges includes dev/build.
    let tree = output(
        "cargo",
        &[
            "tree",
            "--locked",
            "--offline",
            "--workspace",
            "--all-features",
            "--target",
            "all",
            "--edges",
            "normal,build,dev",
            "--prefix",
            "none",
            "--format",
            "{p}",
            "--color",
            "never",
        ],
    )?;
    let root = Path::new(".").canonicalize()?;
    let allowed: Vec<String> = PACKAGES
        .iter()
        .map(|(name, path)| {
            let kind = if *name == "synir-macros" {
                " (proc-macro)"
            } else {
                ""
            };
            format!("{name} v0.1.0{kind} ({})", root.join(path).display())
        })
        .collect();
    for line in tree.lines().filter(|line| !line.trim().is_empty()) {
        if !allowed
            .iter()
            .any(|entry| line == entry || line == format!("{entry} (*)"))
        {
            return Err(format!("unapproved dependency or package path: {line}").into());
        }
    }
    for (_, path) in PACKAGES {
        let manifest = fs::read_to_string(Path::new(path).join("Cargo.toml"))?;
        if !manifest.lines().any(|line| line.trim() == "build = false") {
            return Err(format!("{path} must explicitly disable build scripts").into());
        }
        let lib = if path == "tools/xtask" {
            "src/main.rs"
        } else {
            "src/lib.rs"
        };
        let text = fs::read_to_string(Path::new(path).join(lib))?;
        if !text.contains("#![forbid(unsafe_code)]") {
            return Err(format!("{path} must forbid unsafe code").into());
        }
        if path != "tools/xtask" {
            let readme = fs::read_to_string(Path::new(path).join("README.md"))?;
            if !readme.contains("synir.webp") || !readme.contains("foundation") {
                return Err(format!("missing shared header or scope: {path}").into());
            }
        }
    }
    for path in ["crates/synir/src/lib.rs", "crates/synir-core/src/lib.rs"] {
        if !fs::read_to_string(path)?.contains("#![no_std]") {
            return Err(format!("{path} must be no_std").into());
        }
    }
    walk(Path::new("."))?;
    crate::roadmap::check()?;
    println!("policy: first-party graph, source size, no_std boundaries and local links pass");
    Ok(())
}

fn check_lock(text: &str) -> Result {
    for line in text.lines().map(str::trim) {
        if line.starts_with("source =") || line.starts_with("checksum =") {
            return Err("registry and git dependencies are forbidden, including dev/build".into());
        }
        if let Some(name) = line
            .strip_prefix("name = ")
            .and_then(|s| s.strip_prefix('"'))
            .and_then(|s| s.strip_suffix('"'))
            && !PACKAGES.iter().any(|(allowed, _)| name == *allowed)
        {
            return Err(format!("unapproved package: {name}").into());
        }
    }
    Ok(())
}

fn walk(path: &Path) -> Result {
    for item in fs::read_dir(path)? {
        let item = item?;
        let path = item.path();
        let name = item.file_name();
        if [
            ".git",
            ".agents",
            ".codex",
            "target",
            ".cargo-deny-advisory-dbs",
        ]
        .iter()
        .any(|x| name == *x)
        {
            continue;
        }
        if item.file_type()?.is_symlink() {
            return Err(format!(
                "repository symlink requires policy review: {}",
                path.display()
            )
            .into());
        }
        if path.is_dir() {
            walk(&path)?;
            continue;
        }
        if name == "build.rs" {
            return Err("build scripts are forbidden".into());
        }
        let extension = path.extension().and_then(|x| x.to_str()).unwrap_or("");
        if ["rs", "sh", "py", "yml", "yaml", "toml"].contains(&extension) {
            let text = fs::read_to_string(&path)?;
            if text.lines().count() > 500 {
                return Err(format!("over 500 lines: {}", path.display()).into());
            }
            if path.starts_with("./.github/workflows")
                && text.to_lowercase().contains("codeql-action")
            {
                return Err("use CodeQL Default setup; no advanced workflow".into());
            }
        }
        if extension == "md" {
            links(&path)?;
        }
    }
    Ok(())
}

fn links(path: &Path) -> Result {
    let text = fs::read_to_string(path)?;
    for part in text.split("](").skip(1) {
        let Some((target, _)) = part.split_once(')') else {
            continue;
        };
        if target.contains("://") || target.starts_with('#') || target.starts_with("mailto:") {
            continue;
        }
        let target = target.split('#').next().unwrap_or("");
        if target.is_empty() {
            continue;
        }
        let parent = path.parent().ok_or("missing parent")?;
        if !parent.join(target).exists() {
            return Err(format!("broken local link in {}: {target}", path.display()).into());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::check_lock;

    #[test]
    fn permits_only_known_local_packages() {
        assert!(check_lock("[[package]]\nname = \"synir-core\"\nversion = \"0.1.0\"").is_ok());
        assert!(check_lock("[[package]]\nname = \"third-party\"").is_err());
    }

    #[test]
    fn rejects_registry_and_git_even_with_first_party_name() {
        for source in ["registry+https://example.test", "git+https://example.test"] {
            assert!(check_lock(&format!("name = \"synir-core\"\nsource = \"{source}\"")).is_err());
        }
    }
}

#[cfg(test)]
#[path = "policy_tests.rs"]
mod filesystem_tests;
