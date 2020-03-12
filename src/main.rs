use structopt::*;
use std::path::PathBuf;
use log::*;
mod create;
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
        download_backend: String
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
        Opt::CreateRoot { mirror, target_dir, pacman_config, mirror_list, download_backend } => {
            create::handle(mirror, target_dir, pacman_config, mirror_list, download_backend)
        }
    }
}
