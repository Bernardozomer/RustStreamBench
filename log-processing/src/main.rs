use std::{env, time::SystemTime};

use anyhow::{Context, Result};

mod sequential;
mod rayon;
// mod rust_ssp;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        panic!(
            "Correct usage: $ ./{:?} <runtime> <nthreads> <file name> <ip database filename>",
            args[0]
        );
    }

    let runtime = &args[1];
    let threads = args[2].parse::<usize>().unwrap();
    let filename = &args[3];
    let ip_db_path = &args[4];
    let start = SystemTime::now();

    match runtime.as_str() {
        "sequential" => sequential::parse_log(&filename, &ip_db_path)?,
        "rayon" => rayon::parse_log(&filename, &ip_db_path, threads)?,
        // "rust-ssp" => rust_ssp::parse_log(&filename, threads)?,
        _ => panic!("Invalid runtime, use: sequential | rayon | rust-ssp")
    }

    let system_duration = start
        .elapsed()
        .context("Failed to get render time")?;

    let in_sec = system_duration.as_secs() as f64
        + system_duration.subsec_nanos() as f64 * 1e-9;

    println!("Execution time: {} sec", in_sec);

    Ok(())
}
