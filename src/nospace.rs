use anyhow::Context;
use std::path::PathBuf;
use xshell::{cmd, Shell};

pub(crate) fn run(sh: &Shell) -> anyhow::Result<()> {
    let flags = xflags::parse_or_exit! {
        /// File to rename
        required path: PathBuf
    };
    let full_path = &flags.path;

    let replaced = flags
        .path
        .file_name()
        .and_then(|f| f.to_str())
        .context("basename")?
        .replace(" ", "_");
    let full_replaced_path = flags.path.parent().context("parent path")?.join(replaced);

    cmd!(sh, "mv {full_path} {full_replaced_path}")
        .run()
        .map_err(anyhow::Error::msg)
}
