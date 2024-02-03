use std::path::PathBuf;
use xshell::Shell;

mod bt;
mod yk_gpg;

const TOOLS: &[(&str, fn(&Shell) -> anyhow::Result<()>)] =
    &[("bt", bt::run), ("yk_gpg", yk_gpg::run)];

fn main() -> anyhow::Result<()> {
    let progn: PathBuf = std::env::args_os().next().unwrap_or_default().into();
    let progn = progn
        .file_stem()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();

    if progn == "tools" {
        anyhow::bail!(
            "Available tools: [ {} ]",
            TOOLS
                .iter()
                .copied()
                .map(|(n, _)| n)
                .collect::<Vec<&str>>()
                .join(" "),
        )
    };

    let (_name, run) = TOOLS
        .iter()
        .find(|&&(name, _run)| name == progn)
        .ok_or_else(|| anyhow::format_err!("unknown tool: `{progn}`"))?;

    let sh = Shell::new()?;
    run(&sh)
}

#[cfg(test)]
mod tests {
    use super::*;
    use xshell::{cmd, Shell};

    #[test]
    fn one() {
        let sh = Shell::new().unwrap();
        {
            cmd!(sh, "cargo build --release").run().unwrap();
            for &(name, _) in TOOLS {
            }
        }
    }
}
