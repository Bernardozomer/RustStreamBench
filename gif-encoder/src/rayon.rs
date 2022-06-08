extern crate ffmpeg_next as ffmpeg;

use std::fs::File;
use std::time::SystemTime;

use anyhow::Context as AnyhowContext;
use anyhow::Result;


use { 
    ffmpeg::format::{Pixel, input},
    ffmpeg::frame::Video,
    ffmpeg::media::Type,
    ffmpeg::software::scaling::{Context, Flags},
};

use rayon::prelude::*;

struct DecodedVideo {
    pub ictx: ffmpeg::format::context::Input,
    pub video_stream_index: usize,
    pub decoder: ffmpeg::decoder::Video,
    pub scaler: Context,
}

pub fn encode_gif(filename: &str, threads: usize) -> Result<()> {
    let start = SystemTime::now();

    // Get video data.
    let mut video = decode_video(filename)?;
    let data: Vec<u8> = get_video_data(&mut video, threads)?;
    let width = video.scaler.input().width;
    let height = video.scaler.input().height;

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

fn get_video_data(video: &mut DecodedVideo, threads: usize) -> Result<Vec<u8>> {
    rayon::ThreadPoolBuilder::new().num_threads(threads*5).build_global().unwrap();
    
    let mut pixels: Vec<u8> = Vec::new();
    let mut frame_index = 0;


    let mut receive_and_process_decoded_frames =
        |decoder: &mut ffmpeg::decoder::Video| -> Result<()> {
            let mut decoded = Video::empty();

        while decoder.receive_frame(&mut decoded).is_ok() {
            let mut rgb_frame = Video::empty();
            video.scaler.run(&decoded, &mut rgb_frame)?;
            pixels.extend_from_slice(rgb_frame.data(0));
            frame_index += 1;
        }

        Ok(())
    };

    // Adicionar ideia de um iter()
    // Com o rayon fica iter().par()
    for (stream, packet) in video.ictx.packets() {
        if stream.index() == video.video_stream_index {
            video.decoder.send_packet(&packet)?;
            receive_and_process_decoded_frames(&mut video.decoder)?;
        }
    }

    video.decoder.send_eof()?;
    receive_and_process_decoded_frames(&mut video.decoder)?;

    Ok(pixels)
}

fn decode_video(filename: &str) -> Result<DecodedVideo> {
    let ictx = input(&filename)?;

    let input = ictx
        .streams()
        .best(Type::Video)
        .ok_or(ffmpeg::Error::StreamNotFound)?;
    let video_stream_index = input.index();

    let context_decoder = ffmpeg::codec::context::Context::from_parameters(
        input.parameters()
    )?;

    let decoder = context_decoder.decoder().video()?;

    let scaler = Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        Flags::BILINEAR,
    )?;

    Ok(DecodedVideo {
        ictx,
        video_stream_index,
        decoder,
        scaler,
    })
}
