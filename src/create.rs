use std::path::{Path, PathBuf};

use log::*;
use reqwest::blocking;

use crate::nspawn::{nspawn_run, enter_shell};

pub fn download(mirror: &str, target_directory: &Path, backend: &str) {
    if backend == "pacstrap" {
        std::fs::create_dir_all(target_directory.join("root.x86_64"))
            .and_then(|_| target_directory.join("root.x86_64").canonicalize())
            .and_then(|p| {
                std::process::Command::new("pacstrap")
                    .arg("-c")
                    .arg(p)
                    .arg("base")
                    .arg("base-devel")
                    .spawn()
                    .and_then(|mut x| x.wait().and_then(|x|
                        if x.success() { Ok(()) } else { Err(std::io::Error::new(std::io::ErrorKind::Other, "unexpected exit of pacstrap")) }
                    ))
            })
            .unwrap_or_else(|e| {
                error!("Error: {}", e);
                std::process::exit(1);
            });
        return;
    }
    let bootstrap_url = format!("{}/iso/latest", mirror);
    let responce = blocking::get(bootstrap_url.as_str())
        .map_err(|x| x.to_string())
        .and_then(|x| x.text().map_err(|x| x.to_string()))
        .and_then(|x| regex::Regex::new(r"archlinux-bootstrap-\d{4}\.\d{2}\.\d{2}-x86_64\.tar\.gz")
            .map_err(|x| x.to_string())
            .and_then(|y| y.find(x.as_str()).ok_or(String::from("no matching")))
            .map(|x| String::from(x.as_str())));
    let downloading = match backend {
        "wget" => {
            responce.and_then(|x| {
                let real_url = format!("{}/{}", bootstrap_url, x);
                let target = format!("/tmp/{}", x);
                info!("matched url: {}", real_url);
                std::process::Command::new("wget")
                    .arg(real_url)
                    .arg("--show-progress")
                    .arg("-O")
                    .arg(target.as_str())
                    .spawn()
                    .map_err(|x| x.to_string())
                    .and_then(|mut x| x.wait()
                        .map_err(|x| x.to_string())
                        .and_then(|e| {
                            if e.success() { Ok(()) } else { Err(String::from("unexpected exit of wget")) }
                        }))
                    .map(|_| target)
            })
        }
        "aria2c" => {
            responce.and_then(|x| {
                let real_url = format!("{}/{}", bootstrap_url, x);
                let target = format!("/tmp/{}", x);
                info!("matched url: {}", real_url);
                std::process::Command::new("aria2c")
                    .arg(real_url)
                    .arg("--dir")
                    .arg("/tmp")
                    .arg("--auto-file-renaming=false")
                    .arg("-o")
                    .arg(format!("{}", x))
                    .spawn()
                    .map_err(|x| x.to_string())
                    .and_then(|mut x| x.wait()
                        .map_err(|x| x.to_string())
                        .and_then(|e| {
                            if e.success() { Ok(()) } else { Err(String::from("unexpected exit of aria2c")) }
                        }))
                    .map(|_| target)
            })
        }
        _ => unreachable!()
    };
    let uncompress = downloading.and_then(|target| {
        info!("begin to untar the image");
        std::process::Command::new("tar")
            .arg("-xzf")
            .arg(target)
            .arg("-C")
            .arg(target_directory)
            .spawn()
            .map_err(|x| x.to_string())
            .and_then(|mut x| x.wait()
                .map_err(|x| x.to_string())
                .and_then(|e| {
                    if e.success() { Ok(()) } else { Err(String::from("unexpected exit of tar")) }
                }))
    });
    if let Err(e) = uncompress {
        error!("Error: {}", e);
        std::process::exit(1);
    }
}

pub fn handle(mirror: &String, target_dir: &PathBuf, pacman_config: &Option<PathBuf>, mirror_list: &Option<PathBuf>, download_backend: &String, shell: bool) {
    crate::must_sudo();
    download(mirror, target_dir, download_backend);
    if let Some(config) = pacman_config {
        std::fs::copy(config, format!("{}/root.x86_64/etc/pacman.conf", target_dir.display()))
            .unwrap_or_else(|x| {
                error!("Error: {}, failed to copy config", x);
                std::process::exit(1);
            });
    }
    if let Some(ml) = mirror_list {
        std::fs::copy(ml, format!("{}/root.x86_64/etc/pacman.d/mirrorlist", target_dir.display()))
            .unwrap_or_else(|x| {
                error!("Error: {}, failed to copy mirrorlist", x);
                std::process::exit(1);
            });
    }

    if download_backend != "pacstrap" {
        let key_init = nspawn_run(format!("{}/root.x86_64", target_dir.display()))
            .arg("pacman-key")
            .arg("--init")
            .spawn()
            .map_err(|x| x.to_string())
            .and_then(|mut x| x.wait()
                .map_err(|x| x.to_string())
                .and_then(|e| {
                    if e.success() { Ok(()) } else { Err(String::from("unexpected exit of pacman-key --init")) }
                }));
        let key_populate = key_init.and_then(|_| nspawn_run(format!("{}/root.x86_64", target_dir.display()))
            .arg("pacman-key")
            .arg("--populate")
            .arg("archlinux")
            .spawn()
            .map_err(|x| x.to_string())
            .and_then(|mut x| x.wait()
                .map_err(|x| x.to_string())
                .and_then(|e| {
                    if e.success() { Ok(()) } else { Err(String::from("unexpected exit of pacman-key --init")) }
                })));
        let update = key_populate.and_then(|_| nspawn_run(format!("{}/root.x86_64", target_dir.display()))
            .arg("pacman")
            .arg("-Syu")
            .arg("--noconfirm")
            .spawn()
            .map_err(|x| x.to_string())
            .and_then(|mut x| x.wait()
                .map_err(|x| x.to_string())
                .and_then(|e| {
                    if e.success() { Ok(()) } else { Err(String::from("unexpected exit of pacman -Syu")) }
                })));
        let install = update.and_then(|_|
            nspawn_run(format!("{}/root.x86_64", target_dir.display()))
                .arg("pacman")
                .arg("-S")
                .arg("--noconfirm")
                .arg("base-devel")
                .arg("base")
                .spawn()
                .map_err(|x| x.to_string())
                .and_then(|mut x| x.wait()
                    .map_err(|x| x.to_string())
                    .and_then(|e| {
                        if e.success() { Ok(()) } else { Err(String::from("unexpected exit of pacman -Syu")) }
                    })));
        install.unwrap_or_else(|e| {
            error!("Error: {}", e);
            std::process::exit(1);
        })
    }
    if shell {
        enter_shell(format!("{}/root.x86_64", target_dir.display()));
    }
}