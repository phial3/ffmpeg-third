extern crate ffmpeg_the_third as ffmpeg;

use ffmpeg::{codec, format, frame, media, picture, software};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ffmpeg::init()?;

    let width = 1920;
    let height = 1080;
    let input_file = env::args().nth(1).expect("missing input file");
    let output_file = env::args().nth(2).expect("missing output file");

    // 打开输入/输出文件
    let mut input_context = format::input(&input_file)?;
    let mut output_context = format::output(&output_file)?;

    // 获取视频流信息
    let input_stream = input_context
        .streams()
        .best(media::Type::Video)
        .ok_or("no video stream found")?;
    let input_stream_index = input_stream.index();
    let input_codec_context = codec::context::Context::from_parameters(input_stream.parameters())?;

    // 创建解码器
    let mut decoder = input_codec_context.decoder().video().unwrap();

    // 使用标准的帧率值（如 25fps）
    let frame_rate = ffmpeg::Rational::new(25, 1);
    // 使用标准的时基值（如 1/90000）
    let time_base = ffmpeg::Rational::new(1, 90000);
    println!("frame rate: {:?}, time base: {:?}", frame_rate, time_base);

    // 创建和配置编码器
    let codec = codec::encoder::find(codec::Id::H264);
    let mut encoder = codec::context::Context::new_with_codec(codec.unwrap())
        .encoder()
        .video()?;

    // 设置编码器参数
    encoder.set_height(height);
    encoder.set_width(width);
    encoder.set_aspect_ratio(decoder.aspect_ratio());
    encoder.set_format(decoder.format());
    encoder.set_frame_rate(Some(frame_rate));
    encoder.set_time_base(time_base);
    encoder.set_max_b_frames(0);
    encoder.set_gop(12);
    // 设置一些H.264特定的参数
    encoder.set_bit_rate(2_000_000);

    let mut encoder = encoder.open_as(codec)?;

    // 创建输出流
    let mut output_stream = output_context.add_stream(codec)?;
    output_stream.set_parameters_into(&encoder);
    output_context.write_header()?;

    // 创建缩放器
    let mut scaler = software::scaling::Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        encoder.format(),
        encoder.width(),
        encoder.height(),
        software::scaling::Flags::BILINEAR,
    )?;

    // 创建帧缓冲
    let mut decoded_frame = frame::Video::empty();
    let mut scaled_frame = frame::Video::empty();
    let mut frame_count: i64 = 0;

    println!("video resize start...");

    // 计算每帧的时间增量（以时基为单位）
    // 90000/25 = 3600，每帧增加3600个时基单位
    let time_increment = 90000_i64 / 25;

    // 处理视频帧
    for (stream, packet) in input_context.packets().filter_map(Result::ok) {
        if stream.index() == input_stream_index {
            decoder.send_packet(&packet)?;

            while decoder.receive_frame(&mut decoded_frame).is_ok() {
                // 缩放
                scaler.run(&decoded_frame, &mut scaled_frame)?;

                // 计算正确的时间戳 (pts)
                let pts = frame_count * time_increment;
                scaled_frame.set_pts(Some(pts));
                scaled_frame.set_kind(picture::Type::None);

                // println!("send frame to encoder, pts: {}, time: {:.3}s", pts, pts as f64 / 90000.0);
                encoder.send_frame(&scaled_frame)?;

                // 获取编码后的数据包
                let mut encoded_packet = codec::packet::Packet::empty();
                while encoder.receive_packet(&mut encoded_packet).is_ok() {
                    encoded_packet.set_stream(0);
                    encoded_packet.set_pts(Some(pts));
                    encoded_packet.set_dts(Some(pts));
                    // 写入数据包
                    encoded_packet.write_interleaved(&mut output_context)?;
                }

                frame_count += 1;
            }
        }
    }

    // 刷新编码器
    encoder.send_eof()?;
    let mut packet = codec::packet::Packet::empty();
    while encoder.receive_packet(&mut packet).is_ok() {
        let pts = frame_count * time_increment;
        packet.set_stream(0);
        packet.set_pts(Some(pts));
        packet.set_dts(Some(pts));
        packet.write_interleaved(&mut output_context)?;
        frame_count += 1;
    }

    // 写入文件尾部
    output_context.write_trailer()?;

    println!("video resize complete: {}", output_file);
    println!("total frames processed: {}", frame_count);

    Ok(())
}
