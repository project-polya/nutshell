use structopt::*;
use std::path::PathBuf;
use log::*;
mod create;
mod nspawn;
mod overlay;
#[derive(StructOpt,Debug)]
enum Opt {
    #[structopt(about="create an arch chroot environment")]
    CreateRoot {
        #[structopt(long, short, default_value="https://mirrors.kernel.org/archlinux/", about="image download mirror")]
        mirror: String,
        #[structopt(long, short, about="target directory to create the root")]
        target_dir: PathBuf,
        #[structopt(long, short, about="pacman config file")]
        pacman_config: Option<PathBuf>,
        #[structopt(long, short="l", about="pacman mirror list")]
        mirror_list: Option<PathBuf>,
        #[structopt(long, short, about="download file", default_value="aria2c", possible_values=&["aria2c", "wget", "pacstrap"])]
        download_backend: String,
        #[structopt(long, short, about="enter the chroot environment after basic setups")]
        shell: bool
    },
    #[structopt(about="initialize an overlay file system")]
    InitOverlay {
        #[structopt(long, short, about="the path to mount the merged root")]
        mount_dir: PathBuf,
        #[structopt(long, short, about="the path to the lowerdir")]
        base_dir: PathBuf,
        #[structopt(long, short, about="the path store workdir and upperdir")]
        data_dir: PathBuf,
        #[structopt(long, short, about="create and bind a tmpfs with the given size")]
        tmp_size: Option<String>,
        #[structopt(long, short, about="print the result in json format")]
        print_result: bool,
        #[structopt(long, short, about="enter the chroot environment after mount")]
        shell: bool
    }
}
fn must_sudo() {
    if let Ok(true) = std::env::var("USER").map(|x| "root" == x) {
        return;
    }
    error!("must be root");
    std::process::exit(1);
}
fn main() {
    pretty_env_logger::init();
    let opt = Opt::from_args();
    match &opt {
        Opt::CreateRoot { mirror, target_dir, pacman_config, mirror_list, download_backend, shell } => {
            create::handle(mirror, target_dir, pacman_config, mirror_list, download_backend, *shell);
        },
        Opt::InitOverlay { mount_dir, base_dir, data_dir, tmp_size, print_result, shell } => {
            overlay::handle(mount_dir, base_dir, data_dir, tmp_size, *print_result, *shell);
        }
    }
}
