use clap::Parser;
use ronmmgr::ModsIo;

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, short)]
    steam_dir: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mods_io = ModsIo::default();

    println!("The ModsIo object: {mods_io}");

    match mods_io.mods() {
        Ok(mods) => {
            for m in mods {
                println!("{m}");
            }
        }
        Err(error) => eprintln!("Mods error: {error}"),
    };
}
