use std::{env, time::SystemTime};

use anyhow::{Context, Result};

// mod sequential;
// mod rayon;
// mod rust_ssp;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!(
            "Correct usage: $ ./{:?} <runtime> <nthreads>",
            args[0]
        );
    }

    let runtime = &args[1];
    let threads = args[2].parse::<usize>()?;
    let start = SystemTime::now();

    match runtime.as_str() {
        // "sequential" =>
        // "rayon" =>
        // "rust-ssp" =>
        _ => panic!("Invalid runtime, use: sequential | rayon | rust-ssp")
    }

    let system_duration = start
        .elapsed()
        .context("Failed to get elapsed time")?;

    let in_sec = system_duration.as_secs() as f64
        + system_duration.subsec_nanos() as f64 * 1e-9;

    println!("Execution time: {} sec", in_sec);

    Ok(())
}
