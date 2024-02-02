// This script fixes the issue with Yubikey stored GPG keys where it "caches"
// the saved key locally. If you have multiple yubikeys that are exact copies of
// each other, GPG will think you are using the wrong key.
// See: https://security.stackexchange.com/a/223055

use xshell::{cmd, Shell};

const GPG_KEY_ID: &str = "FF404ABD083C84EC";

pub(crate) fn run(sh: &Shell) -> anyhow::Result<()> {
    let gpg = cmd!(sh, "gpg -K --with-keygrip {GPG_KEY_ID}")
        .quiet()
        .read()?;
    let keys: Vec<_> = gpg
        .lines()
        .filter(|l| l.contains("Keygrip"))
        .filter_map(|l| l.trim().split_once("Keygrip = ").map(|it| it.1))
        .collect();

    for k in keys {
        cmd!(sh, "rm /home/aos/.gnupg/private-keys-v1.d/{k}.key")
            .ignore_status()
            .quiet()
            .run()?;
    }

    cmd!(sh, "gpg --card-status")
        .ignore_status()
        .quiet()
        .run()?;
    Ok(())
}
