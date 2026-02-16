use std::env;

use host_core::config::profile::{Codec, LockPolicy, RuntimeProfile};
use host_core::pipeline::HostCapability;
use host_core::session::SessionManager;

fn main() {
    let profile = match read_profile_from_args() {
        Ok(profile) => profile,
        Err(err) => {
            eprintln!("配置无效: {err}");
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
                "会话已启动：{}x{}@{} 编码={:?} 码率={}kbps",
                started.pipeline.capture.width,
                started.pipeline.capture.height,
                started.pipeline.capture.fps,
                started.pipeline.encoder.codec,
                started.pipeline.encoder.target_bitrate_kbps
            );
        }
        Err(err) => {
            eprintln!("会话启动失败: {err}");
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
                let value = args.get(i).ok_or("--resolution 缺少参数")?;
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
                codec = parse_codec(args.get(i).ok_or("--codec 缺少参数")?)?;
            }
            other => {
                return Err(format!("未知参数: {other}"));
            }
        }
        i += 1;
    }

    RuntimeProfile::new(width, height, fps, bitrate, codec, LockPolicy::TurboLock)
        .map_err(|err| err.to_string())
}

fn parse_u16(value: Option<&String>, key: &str) -> Result<u16, String> {
    value
        .ok_or_else(|| format!("{key} 缺少参数"))?
        .parse::<u16>()
        .map_err(|err| format!("{key} 参数无效: {err}"))
}

fn parse_u32(value: Option<&String>, key: &str) -> Result<u32, String> {
    value
        .ok_or_else(|| format!("{key} 缺少参数"))?
        .parse::<u32>()
        .map_err(|err| format!("{key} 参数无效: {err}"))
}

fn parse_resolution(value: &str) -> Result<(u16, u16), String> {
    let (w, h) = value.split_once('x').ok_or("分辨率格式必须为 宽x高")?;

    let width = w
        .parse::<u16>()
        .map_err(|err| format!("分辨率宽度无效: {err}"))?;
    let height = h
        .parse::<u16>()
        .map_err(|err| format!("分辨率高度无效: {err}"))?;

    Ok((width, height))
}

fn parse_codec(value: &str) -> Result<Codec, String> {
    match value {
        "h264" => Ok(Codec::H264),
        "hevc" => Ok(Codec::Hevc),
        _ => Err("编码格式必须是 h264 或 hevc".to_string()),
    }
}
