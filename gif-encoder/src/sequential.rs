extern crate ffmpeg_next as ffmpeg;

use std::fs::File;
use std::process::Command;
use std::time::SystemTime;

use anyhow::{Context, Result, Ok};
use ffmpeg::format::input;

pub fn encode_gif(filename: &str) -> Result<()> {
    let start = SystemTime::now();

    // Get video data.
    let data: Vec<u8> = get_video_data(&filename)?;
    let (width, height) = get_video_dimensions(&filename)?;

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

    let system_duration = start.elapsed().context("Failed to get render time")?;
    let in_sec = system_duration.as_secs() as f64 + system_duration.subsec_nanos() as f64 * 1e-9;
    println!("Execution time: {} sec", in_sec);

    Ok(())
}

/**
 * Returns a Vec of bytes for every channel in every pixel in every frame
 * of the video. For example, if the first pixel of the first frame is red,
 * the returned vector's first three elements will be 255, 0, and 0.
*/
fn get_video_data(filename: &str) -> Result<Vec<u8>> {
    let cmd = Command::new("ffmpeg")
        .args(["-i", filename, "-pix_fmt", "rgb24", "-f", "rawvideo", "-"])
        .output()
        .context("Failed to extract frames")?;

    Ok(cmd.stdout.into_iter().collect())
}

fn get_video_dimensions(filename: &str) -> Result<(usize, usize)> {
    let input_ctx = input(&filename)?;

    let input = input_ctx
        .streams()
        .best(ffmpeg::media::Type::Video)
        .ok_or(ffmpeg::Error::StreamNotFound)?;

    let decoder = ffmpeg::codec::context::Context::from_parameters(
        input.parameters()
    )?.decoder().video()?;

    Ok((decoder.width().try_into()?, decoder.height().try_into()?))
}
