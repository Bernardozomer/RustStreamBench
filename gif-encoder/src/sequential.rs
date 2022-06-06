extern crate ffmpeg_next as ffmpeg;

use std::fs::File;

use ffmpeg::format::{Pixel, input};
use ffmpeg::frame::Video;
use ffmpeg::media::Type;
use ffmpeg::software::scaling::{Context, Flags};

struct DecodedVideo {
    pub ictx: ffmpeg::format::context::Input,
    pub video_stream_index: usize,
    pub decoder: ffmpeg::decoder::Video,
    pub scaler: Context,
}

pub fn encode_gif(filename: &str) -> Result<(), ffmpeg::Error> {
    let mut video = decode_video(filename)?;
    let pixels: Vec<u8> = dump_pixels_from_video(&mut video)?;

    let mut pixels_per_frame: Vec<&[u8]> = pixels.chunks(
        (video.scaler.input().width * video.scaler.input().height * 3).try_into().unwrap()
    ).collect();

    let mut image = File::create(format!("{}.gif", filename)).unwrap();

    let mut encoder = gif::Encoder::new(
        &mut image,
        video.scaler.input().width.try_into().unwrap(),
        video.scaler.input().height.try_into().unwrap(),
        &[]
    ).unwrap();

    for frame_pixels in pixels_per_frame.iter_mut() {
        let frame = gif::Frame::from_rgb(
            video.scaler.input().width as u16,
            video.scaler.input().height as u16,
            &mut *frame_pixels
        );

        encoder.write_frame(&frame).unwrap();
    }

    Ok(())
}

fn dump_pixels_from_video(video: &mut DecodedVideo) -> Result<Vec<u8>, ffmpeg::Error> {
    let mut pixels: Vec<u8> = Vec::new();
    let mut frame_index = 0;

    dbg!(&video.scaler.input().width);
    dbg!(&video.scaler.input().height);

    let mut receive_and_process_decoded_frames =
        |decoder: &mut ffmpeg::decoder::Video| -> Result<(), ffmpeg::Error> {
            let mut decoded = Video::empty();

            while decoder.receive_frame(&mut decoded).is_ok() {
                let mut rgb_frame = Video::empty();
                video.scaler.run(&decoded, &mut rgb_frame)?;
                pixels.extend_from_slice(rgb_frame.data(0));

                dbg!(&pixels.len());

                frame_index += 1;
            }

            Ok(())
        };

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

fn decode_video(filename: &str) -> Result<DecodedVideo, ffmpeg::Error> {
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

    return Ok(DecodedVideo {
        ictx,
        video_stream_index,
        decoder,
        scaler,
    })
}

