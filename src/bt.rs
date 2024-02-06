use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use xshell::Shell;

// TODO: handle some errors
const PODS: [&str; 2] = ["E0:EB:40:42:4C:B8", "20:15:82:3C:11:DC"];
const TIMEOUT_SEC: usize = 60;

pub(crate) fn run(_: &Shell) -> anyhow::Result<()> {
    println!("Searching for pods: {:?}", PODS);
    if let Some(pod) = find_pod(PODS, TIMEOUT_SEC)? {
        println!("Found pod: {}", pod);

        println!("Pairing...");
        wrap_cmd(&["pair", &pod], "Pairing successful")?;

        println!("Connecting...");
        wrap_cmd(&["connect", &pod], "Connection successful")?;
    } else {
        anyhow::bail!(
            "Failed to find one of {:?} in the last {}",
            PODS,
            TIMEOUT_SEC,
        )
    }
    Ok(())
}

fn find_pod(pods: [&str; 2], timeout: usize) -> anyhow::Result<Option<String>> {
    let mut pod = None;
    let mut cmd = Command::new("/usr/bin/stdbuf")
        .args(["-o0", "bluetoothctl"])
        .args(["scan", "on"])
        .stdout(Stdio::piped())
        .spawn()?;
    let out = cmd
        .stdout
        .take()
        .ok_or_else(|| anyhow::format_err!("Failed to get stdout"))?;

    // set a timeout
    let (tx_timeout, rx_timeout) = mpsc::channel();
    let (tx_found, rx_found) = mpsc::channel();
    let handle = thread::spawn(move || {
        for _ in 0..timeout {
            if rx_found.try_recv().is_ok() {
                return;
            }
            thread::sleep(Duration::from_secs(1));
        }
        tx_timeout.send(true).unwrap();
    });

    // search
    let mut scan_reader = BufReader::new(out);
    let mut line = String::new();
    while scan_reader.read_line(&mut line)? > 0 {
        if let Some(&p) = pods.iter().find(|&&p| line.contains(p)) {
            pod = Some(p.to_owned());
            tx_found.send(true)?;
            break;
        }
        line.clear();
        if rx_timeout.try_recv().is_ok() {
            break;
        }
    }
    cmd.kill()?;
    handle.join().unwrap();
    Ok(pod)
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
