use anyhow::Context;
use xshell::{cmd, Shell};

const DEFAULT_SOURCE: &str = "github:aos/flake-templates";

pub(crate) fn run(sh: &Shell) -> anyhow::Result<()> {
    let flags = xflags::parse_or_exit! {
        /// Use the raw flake reference
        optional -r,--raw
        /// Name of flake template attribute
        required flake_ref: String
    };
    let source = if flags.raw {
        flags.flake_ref
    } else {
        format!("{}#{}", DEFAULT_SOURCE.to_string(), flags.flake_ref)
    };

    cmd!(sh, "nix flake init -t {source}")
        .run()
        .context("run nfi")
}
