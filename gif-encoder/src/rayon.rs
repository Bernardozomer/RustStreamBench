extern crate ffmpeg_next as ffmpeg;

use std::fs::File;

use anyhow::Context as AnyhowContext;
use anyhow::Result;

use rayon::prelude::*;

use crate::core;

pub fn encode_gif(filename: &str, threads: usize) -> Result<()> {
    rayon::ThreadPoolBuilder::new().num_threads(threads).build_global().unwrap();
    // Get video data.
    let data: Vec<u8> = core::get_video_data(&filename)?;
    let (width, height) = core::get_video_dimensions(&filename)?;

    // Separate the video data into frames.
    let data_per_frame: Vec<&[u8]> = data.chunks(
        (width * height * 3).try_into()?
    ).collect();

    // Write frames to file.
    let mut image = File::create(format!("{}.gif", filename))
        .context("Failed to create output file")?;

    let mut encoder = gif::Encoder::new(
        &mut image,
        width.try_into()?,
        height.try_into()?,
        &[],
    )?;

    let frames: Vec<gif::Frame> = data_per_frame.into_iter()
        .par_bridge()
        .map(|frame_data| {
            gif::Frame::from_rgb(
                width.try_into().unwrap(),
                height.try_into().unwrap(),
                frame_data,
            )
        })
        .collect();

    for frame in frames {
        encoder.write_frame(&frame)?;
    }

    Ok(())
}
