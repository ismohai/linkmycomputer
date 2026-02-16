use std::env;

use host_core::config::profile::{Codec, LockPolicy, RuntimeProfile};
use host_core::pipeline::HostCapability;
use host_core::session::SessionManager;

fn main() {
    let profile = match read_profile_from_args() {
        Ok(profile) => profile,
        Err(err) => {
            eprintln!("invalid profile: {err}");
            std::process::exit(2);
        }
    };

    let capability = HostCapability {
        max_width: 2560,
        max_height: 1440,
        max_fps: 144,
        codecs: vec![Codec::Hevc, Codec::H264],
    };

    let mut session = SessionManager::new();
    match session.start(profile, capability) {
        Ok(started) => {
            println!(
                "session started: {}x{}@{} codec={:?} bitrate={}kbps",
                started.pipeline.capture.width,
                started.pipeline.capture.height,
                started.pipeline.capture.fps,
                started.pipeline.encoder.codec,
                started.pipeline.encoder.target_bitrate_kbps
            );
        }
        Err(err) => {
            eprintln!("failed to start session: {err}");
            std::process::exit(3);
        }
    }
}

fn read_profile_from_args() -> Result<RuntimeProfile, String> {
    let mut fps = 144_u16;
    let mut width = 2460_u16;
    let mut height = 1080_u16;
    let mut bitrate = 80_000_u32;
    let mut codec = Codec::Hevc;

    let args = env::args().skip(1).collect::<Vec<_>>();
    let mut i = 0_usize;
    while i < args.len() {
        match args[i].as_str() {
            "--fps" => {
                i += 1;
                fps = parse_u16(args.get(i), "--fps")?;
            }
            "--resolution" => {
                i += 1;
                let value = args.get(i).ok_or("missing value for --resolution")?;
                let (w, h) = parse_resolution(value)?;
                width = w;
                height = h;
            }
            "--bitrate" => {
                i += 1;
                bitrate = parse_u32(args.get(i), "--bitrate")?;
            }
            "--codec" => {
                i += 1;
                codec = parse_codec(args.get(i).ok_or("missing value for --codec")?)?;
            }
            other => {
                return Err(format!("unknown argument: {other}"));
            }
        }
        i += 1;
    }

    RuntimeProfile::new(width, height, fps, bitrate, codec, LockPolicy::TurboLock)
        .map_err(|err| err.to_string())
}

fn parse_u16(value: Option<&String>, key: &str) -> Result<u16, String> {
    value
        .ok_or_else(|| format!("missing value for {key}"))?
        .parse::<u16>()
        .map_err(|err| format!("invalid {key} value: {err}"))
}

fn parse_u32(value: Option<&String>, key: &str) -> Result<u32, String> {
    value
        .ok_or_else(|| format!("missing value for {key}"))?
        .parse::<u32>()
        .map_err(|err| format!("invalid {key} value: {err}"))
}

fn parse_resolution(value: &str) -> Result<(u16, u16), String> {
    let (w, h) = value
        .split_once('x')
        .ok_or("resolution must be WIDTHxHEIGHT")?;

    let width = w
        .parse::<u16>()
        .map_err(|err| format!("invalid resolution width: {err}"))?;
    let height = h
        .parse::<u16>()
        .map_err(|err| format!("invalid resolution height: {err}"))?;

    Ok((width, height))
}

fn parse_codec(value: &str) -> Result<Codec, String> {
    match value {
        "h264" => Ok(Codec::H264),
        "hevc" => Ok(Codec::Hevc),
        _ => Err("codec must be h264 or hevc".to_string()),
    }
}
