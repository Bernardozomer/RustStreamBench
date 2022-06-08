use std::fs::File;

use anyhow::{Context, Result, Ok};
use gif::Frame;
use rust_ssp::*;

use crate::core;

pub fn encode_gif(filename: &str, threads: usize) -> Result<()> {
    // Get video data.
    let data: Vec<u8> = core::get_video_data(&filename)?;
    let (width, height) = core::get_video_dimensions(&filename)?;

    // Separate the video data into frames.
    let mut data_per_frame: Vec<(Vec<u8>, usize)> = Vec::new();
    let mut frame_index = 0;

    for chunk in data.chunks(width * height * 3) {
        data_per_frame.push((chunk.to_vec(), frame_index));
        frame_index += 1;
    }

    // Encode the GIF frames in parallel.
    let pipeline = pipeline![
        parallel!({move |frame_data: (Vec<u8>, usize)|
            Some(
                (
                    Frame::from_rgb(
                        width as u16,
                        height as u16,
                        &frame_data.0,
                    ),
                    frame_data.1
                )
            )
        }, threads as i32),
        collect!()
    ];

    for frame_data in data_per_frame.into_iter() {
        pipeline.post(frame_data).unwrap();
    }

    // Sort the frames into the correct order.
    let mut frames = pipeline.collect();
    frames.sort_unstable_by_key(|tup| tup.1);

    // Write frames to file.
    let mut image = File::create(format!("{}.gif", filename))
        .context("Failed to create output file")?;

    let mut encoder = gif::Encoder::new(
        &mut image,
        width.try_into()?,
        height.try_into()?,
        &[],
    )?;

    for frame in frames.iter() {
        encoder.write_frame(&frame.0)?;
    }

    Ok(())
}
