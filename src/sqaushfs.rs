use std::path::Path;
use log::*;
pub fn handle(source: &Path, target: &Path) {
    let res = std::process::Command::new("mksquashfs")
        .arg(source)
        .arg(target)
        .arg("-comp")
        .arg("lz4")
        .arg("-Xhc")
        .spawn()
        .and_then(|mut x|x.wait())
        .map_err(|x|x.to_string())
        .and_then(|x| if x.success() { Ok(())} else {Err(format!("failed with {}", x))});
    if let Err(e) = res {
        error!("{}", e);
    }
}