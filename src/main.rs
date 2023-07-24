use clap::Parser;
use ronmmgr::ModsIo;

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, short)]
    steam_dir: Option<String>,
}

fn main() {
    let args = Args::parse();
    let mods_io = ModsIo::new()
}
