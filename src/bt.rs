use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use xshell::Shell;

// bt () {
// local choice="${1:-1}"
//
//  bluetoothctl info "${choice}" | grep 'Connected: yes' &>/dev/null || \
//    bluetoothctl connect "${choice}"
//}

// TODO: handle some errors
const PODS: [&str; 2] = ["E0:EB:40:42:4C:B8", "20:15:82:3C:11:DC"];

pub(crate) fn run(_: &Shell) -> anyhow::Result<()> {
    let mut pod = "";

    println!("Scanning for... one of {:?}", PODS);
    let mut cmd = Command::new("/usr/bin/stdbuf")
        .args(["-o0", "bluetoothctl"])
        .args(["scan", "on"])
        .stdout(Stdio::piped())
        .spawn()?;
    let out = cmd
        .stdout
        .take()
        .ok_or_else(|| anyhow::format_err!("Failed to get stdout"))?;

    let mut scan_reader = BufReader::new(out);
    let mut line = String::new();
    while scan_reader.read_line(&mut line)? > 0 {
        if let Some(p) = PODS.iter().find(|&&p| line.contains(p)) {
            pod = p;
            break;
        }
        line.clear();
    }
    println!("found pod: {}", pod);

    println!("Pairing...");
    wrap_cmd(&["pair", &pod], "Pairing successful")?;

    println!("Connecting...");
    wrap_cmd(&["connect", &pod], "Connection successful")?;
    Ok(())
}

fn wrap_cmd(args: &[&str], search: &str) -> anyhow::Result<()> {
    let mut cmd = Command::new("/usr/bin/stdbuf")
        .args(["-o0", "bluetoothctl"])
        .args(args)
        .stdout(Stdio::piped())
        .spawn()?;
    let out = cmd
        .stdout
        .take()
        .ok_or_else(|| anyhow::format_err!("Failed to get stdout"))?;

    let scan_reader = BufReader::new(out);
    for line in scan_reader.lines() {
        let line = line?;
        println!("{}", line);
        if line.contains(search) {
            break;
        }
    }
    cmd.kill()?;
    Ok(())
}
