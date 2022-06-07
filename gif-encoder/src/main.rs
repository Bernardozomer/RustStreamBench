use std::env;

mod sequential;
// mod rust_ssp;
// mod tokio;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        panic!("Correct usage: $ ./{:?} <runtime> <nthreads> <file name>", args[0]);
    }

    let runtime = &args[1];
    // let threads = args[2].parse::<usize>().unwrap();
    let filename = &args[3];

    match runtime.as_str() {
        "sequential" => sequential::encode_gif(&filename).unwrap(),
        // "rust-ssp" => rust_ssp::encode_gif(&filename),
        _ => panic!("Invalid runtime, use: sequential | rust-ssp")
    }

    Ok(())
}
