use rayon::prelude::*;
use xshell::{cmd, Shell};

const BT_CONNECT_TIMEOUT: &str = "10";

pub(crate) fn run(sh: &Shell) -> anyhow::Result<()> {
    let devices = cmd!(sh, "bluetoothctl devices").quiet().read()?;
    let devices = devices
        .split("\n")
        .filter_map(|dev| dev.split(" ").nth(1))
        .collect::<Vec<_>>();

    cmd!(sh, "bluetoothctl power on").run()?;

    let found = devices.par_iter().find_any(|&&x| try_connect(x).is_ok());
    println!("{:?}", found);

    Ok(())
}

fn try_connect(dev: &str) -> anyhow::Result<()> {
    let sh = Shell::new()?;
    cmd!(
        sh,
        "bluetoothctl --timeout {BT_CONNECT_TIMEOUT} connect {dev}"
    )
    .quiet()
    .run()
    .map_err(anyhow::Error::msg)
}
