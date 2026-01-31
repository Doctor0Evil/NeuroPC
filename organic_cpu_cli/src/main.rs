use organic_cpu_orchestrator::ffi_json::run_ffi_once;
use organic_cpu_profile::load_profile_from_file;
use std::env;

fn main() {
    // Args: <profile_dir> <host_id> <safe_mode> <session_tag>
    // Example: organic_cpu_cli ./profiles "nvim-neuropc" true "0xNP0A"
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!(
            "Usage: {} <profile_dir> <host_id> <safe_mode> <session_tag>",
            args[0]
        );
        std::process::exit(1);
    }

    let profile_dir = &args[1];
    let host_id = &args[2];
    let safe_mode = args[3].parse::<bool>().unwrap_or(true);
    let session_tag = &args[4];

    let loader = move |profile_id: &str| {
        let path = format!("{}/{}.ocpu", profile_dir, profile_id);
        load_profile_from_file(&path)
    };

    if let Err(e) = run_ffi_once(&loader, host_id, safe_mode, session_tag) {
        eprintln!("organic_cpu_cli error: {e:?}");
        std::process::exit(1);
    }
}
