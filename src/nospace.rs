use anyhow::Context;
use std::path::PathBuf;
use xshell::{cmd, Shell};

pub(crate) fn run(sh: &Shell) -> anyhow::Result<()> {
    let flags = xflags::parse_or_exit! {
        /// File to rename
        required path: PathBuf
    };
    let curr_path = flags.path.to_str().context("path to string")?;
    let replaced = curr_path.replace(" ", "_");

    cmd!(sh, "mv {curr_path} {replaced}")
        .run()
        .map_err(anyhow::Error::msg)
}
