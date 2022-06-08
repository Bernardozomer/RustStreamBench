use std::{env, time::SystemTime};

use anyhow::{Context, Result};

mod core;
mod sequential;
mod rayon;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        panic!(
            "Correct usage: $ ./{:?} <runtime> <nthreads> <file name>",
            args[0]
        );
    }

    let runtime = &args[1];
    let threads = args[2].parse::<usize>().unwrap();
    let filename = &args[3];
    let start = SystemTime::now();

    match runtime.as_str() {
        "sequential" => sequential::encode_gif(&filename)?,
        "rayon" => rayon::encode_gif(&filename, threads)?,
        _ => panic!("Invalid runtime, use: sequential | rust-ssp")
    }

    let system_duration = start
        .elapsed()
        .context("Failed to get render time")?;
    let in_sec = system_duration.as_secs() as f64 + system_duration.subsec_nanos() as f64 * 1e-9;
    println!("Execution time: {} sec", in_sec);

    Ok(())
}
