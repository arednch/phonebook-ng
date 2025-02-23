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

    let sysinfo = sysinfo::load_sysinfo(&args.sysinfo).expect("error loading sysinfo");
    let host_map = sysinfo.create_host_map();

    let sources = args.sources.split(",");
    for s in sources {
        let source = s.trim();

        let records = loader::load_phonebook(&source, &host_map).expect("error loading phonebook");
        println!("loaded {} records.", records.len());

        exporter::export_phonebook(&records, &args.path).expect("error exporting to XML");
        println!("exported XML to {:?}", args.path);

        // Stop after the first successful processing of a phonebook
        break;
    }
}
