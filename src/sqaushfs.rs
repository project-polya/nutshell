use std::path::Path;

use log::*;

pub fn handle(source: &Path, target: &Path, faster: bool) {
    let mut command = std::process::Command::new("mksquashfs");
    command.arg(source)
        .arg(target)
        .arg("lz4");
    if faster { command.arg("-Xhc"); }
    let res = command
        .spawn()
        .and_then(|mut x| x.wait())
        .map_err(|x| x.to_string())
        .and_then(|x| if x.success() { Ok(()) } else { Err(format!("failed with {}", x)) });
    if let Err(e) = res {
        error!("{}", e);
    }
}