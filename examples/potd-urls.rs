
extern crate potd;

use clap::Parser;
use serde_json::Value;

/// Example POTD client to fetch and display POTD URLs
#[derive(Parser, Debug)]
#[command(version, about = "Example POTD client to fetch and display POTD URLs", long_about = None)]
struct Opts {
    /// Target width of the image
    #[clap(short, long, default_value = "1920")]
    target_width: usize,

    /// Output in JSON format
    #[clap(short, long)]
    json: bool,
}

fn main() {
    let opts: Opts = Opts::parse();
    let engine = potd::Engine::new(opts.target_width);
    let urls = engine.run_blocking().unwrap();
    if opts.json {
        let urls = urls.iter().map(|url| Value::String(url.to_string())).collect::<Vec<Value>>();
        println!("{}", Value::Array(urls).to_string());
    } else {
        for url in urls {
            println!("{}", url);
        }
    }
}
