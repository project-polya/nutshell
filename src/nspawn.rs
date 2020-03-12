use std::process::Command;
use log::*;
use std::ffi::OsStr;

pub fn nspawn_run<A : AsRef<OsStr>>(target_dir: A) -> Command {
    let mut command = Command::new("systemd-nspawn");
    command
        .arg("--quiet")
        .arg("--resolv-conf=bind-host")
        .arg("-D")
        .arg(target_dir.as_ref());
    command
}

pub fn enter_shell<A : AsRef<OsStr>>(target_dir: A) {
    nspawn_run(target_dir)
        .spawn()
        .map_err(|x| x.to_string())
        .and_then(|mut x| x.wait()
            .map_err(|x| x.to_string())
            .and_then(|e| {
                if e.success() { Ok(()) } else { Err(String::from("unexpected exit of shell mode")) }
            }))
        .unwrap_or_else(|e| {
            error!("Error: {}", e);
            std::process::exit(1);
        });
}