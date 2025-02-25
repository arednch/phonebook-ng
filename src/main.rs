mod exporter;
mod loader;
mod sysinfo;

use core::time;
use std::{arch::aarch64::int32x2x4_t, thread};

use clap::Parser;

/// AREDN Phonebook
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Comma separated paths or URLs to fetch the phonebook CSV from.
    #[arg(short, long, required = true)]
    sources: String,

    /// Folder to write the phonebooks to locally.
    #[arg(short, long, required = true)]
    path: String,

    /// URL of sysinfo JSON API.
    #[arg(
        long,
        required = true,
        default_value = "http://localnode.local.mesh/cgi-bin/sysinfo.json?hosts=1"
    )]
    sysinfo: String,

    /// Phonebook runs as a service / daemon when set to true.
    #[arg(long, default_value_t = false, required = false)]
    server: bool,

    /// Duration in seconds after which to try to reload and export the phonebook source.
    #[arg(long, default_value_t = 3600, required = false)]
    reload: u32,
}

fn main() {
    let args = Args::parse();

    if args.reload < 10 {
        eprintln!("reload delay is less than 10 seconds");
        std::process::exit(1);
    }
    let delay = time::Duration::from_secs(args.reload.into());

    // TODO: Start a simple SIP server.
    // - must: register returns an OK
    // - optimally: redirects calls

    loop {
        let sysinfo = sysinfo::load_sysinfo(&args.sysinfo).expect("error loading sysinfo");
        let host_map = sysinfo.create_host_map();

        let sources = args.sources.split(",");
        for s in sources {
            let source = s.trim();

            let records =
                loader::load_phonebook(&source, &host_map).expect("error loading phonebook");
            println!("loaded {} records.", records.len());

            exporter::export_phonebook(&records, &args.path).expect("error exporting to XML");
            println!("exported XML to {:?}", args.path);

            // Stop after the first successful processing of a phonebook
            break;
        }

        if !args.server {
            break;
        }
        println!("sleeping for {} seconds", delay.as_secs());
        thread::sleep(delay);
    }
}
