# NutShell

![nutshell](logo.png)

Create a judge environment in a **nutshell**! 

Nutshell is a part of [Project Polya](https://github.com/project-polya). This project means to provide a
CLI helper for teachers/students to set up their own sandboxed environment, so that they can work together
smoothly in later grading.

## How to use?
Why not check the help page first?
```text
nutshell 0.1.0

USAGE:
    nutshell <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    create-root      create an arch chroot environment
    help             Prints this message or the help of the given subcommand(s)
    init-overlay     initialize an overlay file system
    make-squashfs    create a squashfs
```

### create-root
```text
nutshell-create-root 0.1.0
create an arch chroot environment

USAGE:
    nutshell create-root [FLAGS] [OPTIONS] --target-dir <target-dir>

FLAGS:
    -h, --help       Prints help information
    -s, --shell      enter the chroot environment after basic setups
    -V, --version    Prints version information

OPTIONS:
    -d, --download-backend <download-backend>    downloader [default: aria2c]  [possible values: aria2c, wget, pacstrap]
    -m, --mirror <mirror>                        image download mirror [default: https://mirrors.kernel.org/archlinux/]
    -l, --mirror-list <mirror-list>              pacman mirror list
    -p, --pacman-config <pacman-config>          pacman config file
    -t, --target-dir <target-dir>                target directory to create the root              
```
This command helps you to create an Arch Linux chroot environment. You can set your own download backend:
`aria2c`, `wget`, `pacstrap` are all available.

This command will download the bootstrap tar of Arch and then extract it into the target directory. Using `systemd-nspawn`,
it will populate GPG keys and run some basic updates. It will also prepare the `base` and `base-devel` environment.

#### For Users from Mainland China
You are highly recommeneded to set a download mirror at mainland, such as `https://mirrors.tuna.tsinghua.edu.cn/archlinux/`.
As for the `mirrorlist`, you may want some configurations like the following:
```text
# mirrorlist
Server = https://ftp.sjtu.edu.cn/archlinux/$repo/os/$arch
Server = https://mirrors.ustc.edu.cn/archlinux/$repo/os/$arch
Server = https://mirrors.tuna.tsinghua.edu.cn/archlinux/$repo/os/$arch
```

### init-overlay
```text
nutshell-init-overlay 0.1.0
initialize an overlay file system

USAGE:
    nutshell init-overlay [FLAGS] [OPTIONS] --base-dir <base-dir> --data-dir <data-dir> --mount-dir <mount-dir>

FLAGS:
    -h, --help            Prints help information
    -p, --print-result    print the result in json format
    -s, --shell           enter the chroot environment after mount
    -V, --version         Prints version information

OPTIONS:
    -b, --base-dir <base-dir>      the path to the lowerdir
    -d, --data-dir <data-dir>      the path store workdir and upperdir
    -m, --mount-dir <mount-dir>    the path to mount the merged root
    -t, --tmp-size <tmp-size>      create and bind a tmpfs with the given size
``` 
This command helps to mount a squashfs file with an overlay layer. `data-dir` and `base-dir` are the temporary storage path needed for overlay layer.

### make-squashfs
```text
nutshell-make-squashfs 0.1.0
create a squashfs

USAGE:
    nutshell make-squashfs [FLAGS] --source <source> --target <target>

FLAGS:
    -f, --faster     make the process faster by disabling high quality compression
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --source <source>    source directory
    -t, --target <target>    target path
```
This command is just a wrapper of `mksquashfs`. It will help to create a squashfs file that can be used
in the Project Polya. 

By default, it uses `-comp lz4 -Xhc` as the compression args. 

## BTW, we use Arch Linux
![arch logo](arch.png)
We find that Arch Linux enjoy lots of up to date packages and a very lighweight design principle,
which make it a very ideal Linux distro for Project Polya.