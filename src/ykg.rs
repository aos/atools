// This script fixes the issue with Yubikey stored GPG keys where it "caches"
// the saved key locally. If you have multiple yubikeys that are exact copies of
// each other, GPG will think you are using the wrong key.
// See: https://security.stackexchange.com/a/223055

use xshell::{cmd, Shell};

pub(crate) fn run(sh: &Shell) -> anyhow::Result<()> {
    cmd!(sh, "gpg-connect-agent 'scd serialno' 'learn --force' /bye")
        .run()
        .map_err(|e| anyhow::format_err!("Failed to fix GPG key: {e}"))
}
