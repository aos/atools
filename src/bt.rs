use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use xshell::Shell;

// bt () {
// local choice="${1:-1}"
//
//  case "$choice" in
//    1) choice="E0:EB:40:42:4C:B8"
//      ;;
//    2) choice="20:15:82:3C:11:DC"
//      ;;
//  esac
//
//  bluetoothctl info "${choice}" | grep 'Connected: yes' &>/dev/null || \
//    bluetoothctl connect "${choice}"
//}

const PODS: [&str; 2] = ["E0:EB:40:42:4C:B8", "20:15:82:3C:11:DC"];

pub(crate) fn run(_: &Shell) -> anyhow::Result<()> {
    let mut pod = "";
    let wait_timeout = 60;

    println!("Scanning for... one of {:?}", PODS);
    let mut cmd = Command::new("bluetoothctl")
        .args(["scan", "on"])
        .stdout(Stdio::piped())
        .spawn()?;
    let out = cmd
        .stdout
        .take()
        .ok_or_else(|| anyhow::format_err!("Failed to get stdout"))?;

    // 30 second timeout
    let thread = thread::spawn(move || {
        for _ in 0..wait_timeout {
            if let Ok(Some(_)) = cmd.try_wait() {
                return;
            }
            thread::sleep(Duration::from_secs(1));
        }
        cmd.kill().expect("Unable to kill child process");
    });

    let scan_reader = BufReader::new(out);
    for line in scan_reader.lines() {
        let line = line?;
        if let Some(p) = PODS.iter().find(|&&p| line.contains(p)) {
            pod = p;
            break;
        }
    }

    thread.join().unwrap();
    if pod.is_empty() {
        anyhow::bail!(
            "Failed to find any pods in the last {} seconds... exiting.",
            wait_timeout
        );
    }

    println!("found pod: {}", pod);

    println!("Pairing...");
    wrap_cmd(&["pair", &pod], "Pairing successful")?;

    println!("Connecting...");
    wrap_cmd(&["connect", &pod], "Connection successful")?;
    Ok(())
}

fn wrap_cmd(args: &[&str], search: &str) -> anyhow::Result<()> {
    let mut cmd = Command::new("bluetoothctl")
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
