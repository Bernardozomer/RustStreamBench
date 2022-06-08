extern crate ffmpeg_next as ffmpeg;

use std::fs::File;

use anyhow::{Context, Result, Ok};

use crate::core;

pub fn encode_gif(filename: &str) -> Result<()> {
    // Get video data.
    let data: Vec<u8> = core::get_video_data(&filename)?;
    let (width, height) = core::get_video_dimensions(&filename)?;

    // Separate the video data into frames.
    let mut data_per_frame: Vec<&[u8]> = data.chunks(
        (width * height * 3).try_into()?
    ).collect();

    // Encode the GIF frames and write them to a file.
    let mut image = File::create(format!("{}.gif", filename))
        .context("Failed to create output file")?;

    let mut encoder = gif::Encoder::new(
        &mut image,
        width.try_into()?,
        height.try_into()?,
        &[],
    )?;

    for frame_data in data_per_frame.iter_mut() {
        let frame = gif::Frame::from_rgb(
            width.try_into()?,
            height.try_into()?,
            frame_data,
        );

        encoder.write_frame(&frame)?;
    }

    Ok(())
}
