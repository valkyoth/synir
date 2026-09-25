//! First-party repository checks. No third-party Cargo dependencies.
#![forbid(unsafe_code)]

mod policy;
mod review;
mod roadmap;

use std::{env, error::Error, path::Path, process::Command};

type Result<T = ()> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    env::set_current_dir(root.canonicalize()?)?;
    let args: Vec<String> = env::args().skip(1).collect();
    match args.as_slice() {
        [command] if command == "policy" => policy::check(),
        [command] if command == "freshness" => review::freshness(),
        _ => Err("usage: cargo xtask {policy|freshness}".into()),
    }
}

fn output(program: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(program).args(args).output()?;
    if !output.status.success() {
        return Err(format!(
            "{program} {args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    Ok(String::from_utf8(output.stdout)?)
}
