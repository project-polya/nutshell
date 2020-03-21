use std::path::PathBuf;

use log::*;
use serde::*;

use crate::nspawn::enter_shell;

#[derive(Debug, Serialize, Deserialize)]
struct MountResult {
    workdir: PathBuf,
    upperdir: PathBuf,
    with_tmp: bool,
}

pub fn handle(mount_dir: &PathBuf, base_dir: &PathBuf, data_dir: &PathBuf, tmp_size: &Option<String>, print_result: bool, shell: bool) {
    crate::must_sudo();
    let upperdir = data_dir.join("upperdir");
    let workdir = data_dir.join("workdir");
    let prepare = std::fs::create_dir_all(upperdir.as_path())
        .and_then(|_| std::fs::create_dir_all(workdir.as_path()))
        .map_err(|x| x.to_string());
    prepare.and_then(|_|
        std::process::Command::new("mount")
            .arg("-t")
            .arg("squashfs")
            .arg(base_dir.as_path())
            .arg(mount_dir.as_path())
            .spawn()
            .and_then(|mut x| x.wait())
            .map_err(|x| x.to_string())
    ).and_then(|_|
        std::process::Command::new("mount")
            .arg("-t")
            .arg("overlay")
            .arg("overlay")
            .arg("-o")
            .arg(format!("lowerdir={},upperdir={},workdir={}", mount_dir.display(), upperdir.display(), workdir.display()))
            .arg(mount_dir.as_path())
            .spawn()
            .map_err(|x| x.to_string())
            .and_then(|mut x| x.wait()
                .map_err(|x| x.to_string())
                .and_then(|e| {
                    if e.success() { Ok(()) } else { Err(String::from("failed to mount")) }
                })))
        .unwrap_or_else(|x| {
            error!("Error: {}, failed to mount overlayfs", x);
            std::process::exit(1);
        });
    if let Some(t) = tmp_size {
        std::process::Command::new("mount")
            .arg("-t")
            .arg("tmpfs")
            .arg("-o")
            .arg(format!("size={}", t))
            .arg("tmpfs")
            .arg(format!("{}/tmp", mount_dir.display()))
            .spawn()
            .map_err(|x| x.to_string())
            .and_then(|mut x| x.wait()
                .map_err(|x| x.to_string())
                .and_then(|e| {
                    if e.success() { Ok(()) } else { Err(String::from("failed to mount")) }
                }))
            .unwrap_or_else(|x| {
                error!("Error: {}, failed to mount overlayfs", x);
                std::process::exit(1);
            });
    }
    std::fs::create_dir_all(format!("{}/tmp/.X11-unix", mount_dir.display()))
        .unwrap_or_else(|x| {
            error!("Error: {}, failed to create X11 temporary dir", x);
            std::process::exit(1);
        });
    if shell {
        enter_shell(mount_dir);
    }
    if print_result {
        println!(
            "{}", serde_json::to_string(&MountResult {
                workdir,
                upperdir,
                with_tmp: tmp_size.is_some(),
            }).unwrap_or_else(|x| {
                error!("Error: {}, failed to generate the result", x);
                std::process::exit(1);
            })
        )
    }
}