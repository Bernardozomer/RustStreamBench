extern crate ffmpeg_next as ffmpeg;

use std::process::Command;

use anyhow::{Context, Result, Ok};
use ffmpeg::format::input;

/**
 * Returns a Vec of bytes for every channel in every pixel in every frame
 * of the video. For example, if the first pixel of the first frame is red,
 * the returned vector's first three elements will be 255, 0, and 0.
*/
pub fn get_video_data(filename: &str) -> Result<Vec<u8>> {
    let cmd = Command::new("ffmpeg")
        .args(["-i", filename, "-pix_fmt", "rgb24", "-f", "rawvideo", "-"])
        .output()
        .context("Failed to extract frames")?;

    Ok(cmd.stdout.into_iter().collect())
}

pub fn get_video_dimensions(filename: &str) -> Result<(usize, usize)> {
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
