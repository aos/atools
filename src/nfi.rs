use anyhow::Context;
use xshell::{cmd, Shell};

const DEFAULT_SOURCE: &str = "github:aos/flake-templates";

pub(crate) fn run(sh: &Shell) -> anyhow::Result<()> {
    let flags = xflags::parse_or_exit! {
        /// Different source
        optional -s,--source source: String
        /// File to rename
        required name: String
    };
    let name = flags.name;
    let source = flags.source.unwrap_or(DEFAULT_SOURCE.to_string());

    cmd!(sh, "nix flake init -t {source}#{name}")
        .run()
        .context("run nfi")
}
