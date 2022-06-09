use std::fs::File;

use anyhow::{Context, Result};
use rayon::prelude::*;

use crate::core;

pub fn encode_gif(filename: &str, threads: usize) -> Result<()> {
    rayon::ThreadPoolBuilder::new().num_threads(threads).build_global()?;
    // Get video data.
    let data: Vec<u8> = core::get_video_data(&filename)?;
    let (width, height) = core::get_video_dimensions(&filename)?;

    // Separate the video data into frames.
    let data_per_frame: Vec<&[u8]> = data.chunks(
        (width * height * 3).try_into()?
    ).collect();

    // Encode the GIF frames in parallel.
    let frames: Vec<gif::Frame> = data_per_frame.into_iter()
        .par_bridge()
        .map(|frame_data: &[u8]| {
            gif::Frame::from_rgb(
                width as u16,
                height as u16,
                frame_data,
            )
        })
        .collect();

    // Write frames to file.
    let mut image = File::create(format!("{}.gif", filename))
        .context("Failed to create output file")?;

    let mut encoder = gif::Encoder::new(
        &mut image,
        width as u16,
        height as u16,
        &[],
    )?;

    for frame in frames {
        encoder.write_frame(&frame)?;
    }

    Ok(())
}
