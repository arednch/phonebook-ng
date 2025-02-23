mod exporter;
mod loader;
mod sysinfo;

use clap::Parser;

/// AREDN Phonebook
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Comma separated paths or URLs to fetch the phonebook CSV from.
    #[arg(short, long)]
    sources: String,

    /// Folder to write the phonebooks to locally.
    #[arg(short, long)]
    path: String,

    /// URL of sysinfo JSON API.
    #[arg(
        long,
        default_value = "http://localnode.local.mesh/cgi-bin/sysinfo.json?hosts=1"
    )]
    sysinfo: String,
}

fn main() {
    let args = Args::parse();

    let sysinfo = match sysinfo::load_sysinfo(&args.sysinfo) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("error loading sysinfo: {}", e);
            std::process::exit(1);
        }
    };
    let host_map = sysinfo.create_host_map();

    let sources = args.sources.split(",");
    for s in sources {
        let source = s.trim();

        match loader::load_phonebook(&source, &host_map) {
            Ok(records) => {
                println!("loaded {} records.", records.len());
                if let Err(e) = exporter::export_phonebook(&records, &args.path) {
                    eprintln!("error exporting to XML: {}", e);
                    std::process::exit(1);
                }
                println!("exported XML to {:?}", args.path);
                break; // Stop after the first successful processing of a phonebook
            }
            Err(e) => {
                eprintln!("error loading phonebook: {}", e);
                std::process::exit(1);
            }
        }
    }
}
