use std::{env, time::SystemTime};

use anyhow::{Context, Result};

mod core;
mod sequential;
// mod rayon;
// mod rust_ssp;

const MOMENTUM: f64 = 0.05;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 6 {
        panic!(
            "Correct usage: $ ./{:?} <runtime> <nthreads> <app> <learning rate> <iterations>",
            args[0]
        );
    }

    let runtime = &args[1];
    let threads = args[2].parse::<usize>()?;
    let app = &args[3];
    let learning_rate = args[4].parse::<f64>()?;
    let iterations = args[6].parse::<i64>()?;
    let start = SystemTime::now();

    let architecture: &[i32] = match app.as_str() {
        "segmentation" => &[19, 38, 38, 1],
        _ => panic!("Invalid app, use: segmentation"),
    };

    match runtime.as_str() {
        "sequential" => sequential::run(
            app, architecture, learning_rate, MOMENTUM, iterations
        )?,
        // "rayon" => rayon::run(
        //     app, architecture, learning_rate, MOMENTUM, iterations, threads
        // )?,
        // "rust-ssp" => rust_ssp::run(
        //     app, architecture, learning_rate, MOMENTUM, iterations, threads
        // )?,
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
