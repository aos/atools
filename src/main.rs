use std::path::PathBuf;
use xshell::Shell;

mod bt;
mod btw;
mod nfi;
mod nospace;
mod ykg;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const TOOLS: &[(&str, fn(&Shell) -> anyhow::Result<()>)] = &[
    // ("bt", bt::run),
    ("btw", btw::run),
    ("ykg", ykg::run),
    ("nfi", nfi::run),
    ("nospace", nospace::run),
];

fn main() -> anyhow::Result<()> {
    let progn: PathBuf = std::env::args_os().next().unwrap_or_default().into();
    let progn = progn
        .file_stem()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();

    if progn == PKG_NAME {
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
    use super::TOOLS;
    use xshell::{cmd, Shell};

    #[test]
    fn link() -> anyhow::Result<()> {
        let sh = Shell::new().unwrap();

        cmd!(sh, "cargo clean").run().unwrap();
        cmd!(sh, "cargo build --release").run().unwrap();
        for &(name, _) in TOOLS {
            sh.hard_link(
                "./target/release/atools",
                format!("./target/release/{name}"),
            )?;
        }
        Ok(())
    }
}
