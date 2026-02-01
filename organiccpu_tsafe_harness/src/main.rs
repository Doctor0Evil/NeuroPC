use organiccpu_tsafe_harness::run::{run_once, HarnessConfig};
use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: organiccpu_tsafe_harness <kernel.yaml> <x0,...,xN-1>");
        std::process::exit(1);
    }

    let kernel_path = PathBuf::from(&args[1]);
    let components: Vec<f64> = args[2]
        .split(',')
        .map(|s| s.trim().parse::<f64>())
        .collect::<Result<_, _>>()
        .unwrap_or_else(|e| {
            eprintln!("failed to parse state vector: {e}");
            std::process::exit(1);
        });

    let cfg = HarnessConfig {
        kernel_path: &kernel_path,
        fake_state: components,
    };

    match run_once(cfg) {
        Ok(true) => {
            println!("SAFE");
        }
        Ok(false) => {
            println!("UNSAFE");
            std::process::exit(2);
        }
        Err(e) => {
            eprintln!("HARNESS ERROR: {e}");
            std::process::exit(3);
        }
    }
}
