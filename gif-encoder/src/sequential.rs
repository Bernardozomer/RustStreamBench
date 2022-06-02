extern crate ffmpeg_next as ffmpeg;

use std::fs::File;

use ffmpeg::format::{Pixel, input};
use ffmpeg::frame::Video;
use ffmpeg::media::Type;
use ffmpeg::software::scaling::{Context, Flags};

pub fn encode_gif(filename: &str) -> Result<(), ffmpeg::Error> {
    // Get pixel data from video
    let mut pixels: Vec<u8> = dump_pixels_from_video(filename)?;
    // Create frame from data
    let frame = gif::Frame::from_rgb(1280, 720, &mut *pixels);
    // Create encoder
    let mut image = File::create(format!("{}.gif", filename)).unwrap();
    let mut encoder = gif::Encoder::new(&mut image, frame.width, frame.height, &[]).unwrap();
    // Write frame to file
    encoder.write_frame(&frame).unwrap();

    Ok(())
}

fn dump_pixels_from_video(filename: &str) -> Result<Vec<u8>, ffmpeg::Error> {
    let mut ictx = input(&filename)?;

    let input = ictx
        .streams()
        .best(Type::Video)
        .ok_or(ffmpeg::Error::StreamNotFound)?;
    let video_stream_index = input.index();

    let context_decoder = ffmpeg::codec::context::Context::from_parameters(
        input.parameters()
    )?;

    let mut decoder = context_decoder.decoder().video()?;

    let mut scaler = Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        Flags::BILINEAR,
    )?;

    let mut pixels: Vec<u8> = Vec::new();
    let mut frame_index = 0;

    let mut receive_and_process_decoded_frames =
        |decoder: &mut ffmpeg::decoder::Video| -> Result<(), ffmpeg::Error> {
            let mut decoded = Video::empty();
            while decoder.receive_frame(&mut decoded).is_ok() {
                let mut rgb_frame = Video::empty();
                scaler.run(&decoded, &mut rgb_frame)?;

                if pixels.len() <= 0 {
                    pixels.extend_from_slice(rgb_frame.data(0));
                }

                frame_index += 1;
            }

            Ok(())
        };

    for (stream, packet) in ictx.packets() {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet)?;
            receive_and_process_decoded_frames(&mut decoder)?;
        }
    }

    decoder.send_eof()?;
    receive_and_process_decoded_frames(&mut decoder)?;

    Ok(pixels)
}
