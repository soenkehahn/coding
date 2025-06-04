mod niri;
mod programs;
mod sway;
mod utils;

use std::path::Path;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {}

fn main() {
    let wm_type = WmType::get();
    let vcs_type = VcsType::get();
    eprintln!("{:?}, {:?}", wm_type, vcs_type);
    let _args = Args::from_args();
    match wm_type {
        WmType::Sway => sway::setup(vcs_type),
        WmType::Niri => niri::setup(),
    }
    eprintln!("Done.");
}

#[derive(Debug, Clone, Copy)]
enum WmType {
    Sway,
    Niri,
}

impl WmType {
    fn get() -> WmType {
        match (
            std::env::var_os("SWAYSOCK"),
            std::env::var_os("NIRI_SOCKET"),
        ) {
            (Some(_), None) => WmType::Sway,
            (None, Some(_)) => WmType::Niri,
            _ => panic!("cannot detect window manager"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum VcsType {
    None,
    Git,
    Jujutsu,
}

impl VcsType {
    fn get() -> VcsType {
        if Path::new("./.jj").exists() {
            VcsType::Jujutsu
        } else if Path::new("./.git").exists() {
            VcsType::Git
        } else {
            VcsType::None
        }
    }
}
