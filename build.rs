use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("./configure")
        .current_dir("ffmpeg")
        .arg(format!("--prefix={}", out_dir))
        .args(common_configs())
        .output()
        .expect("Failed to run ffmpeg's configure script");

    Command::new("make")
        .current_dir("ffmpeg")
        .arg("-j")
        .arg("8")
        .output()
        .expect("Failed to run ffmpeg's Makefile build step");

    Command::new("make")
        .current_dir("ffmpeg")
        .arg("install")
        .output()
        .expect("Failed to run ffmpeg's Makefile install step");

    Command::new("bindgen")
        .arg("--use-core")
        .arg("--ctypes-prefix=crate::ctypes")
        .arg("--no-doc-comments")
        .arg("--with-derive-default")
        .arg("--whitelist-function")
        .arg("avcodec.*")
        .arg("--whitelist-function")
        .arg("av_.*")
        .arg("--whitelist-function")
        .arg("avformat_.*")
        .arg("--whitelist-function")
        .arg("avio_.*")
        .arg("--whitelist-type")
        .arg("avcodec.*")
        .arg("--whitelist-type")
        .arg("AV_CODEC_.*")
        .arg("--whitelist-type")
        .arg("avformat.*")
        .arg("--whitelist-type")
        .arg("avformat.*")
        .arg("--whitelist-type")
        .arg("avio.*")
        .arg("bindgen/wrapper.h")
        .arg("-o")
        .arg(format!("{}/bindings.rs", out_dir))
        .arg("--")
        .arg(format!("-I{}/include", out_dir))
        .output()
        .expect("Failed to run bindgen");

    println!("cargo:rerun-if-changed=bindgen/wrapper.h");
    println!("cargo:rerun-if-env-changed=CC");
    println!("cargo:rustc-link-search=native={}/lib", out_dir);

    println!("cargo:rustc-link-lib=static=avcodec");
    println!("cargo:rustc-link-lib=static=avutil");
}

fn common_configs() -> &'static [&'static str] {
    &[
        "--disable-everything",
        "--disable-programs",
        "--disable-doc",
        "--disable-debug",
        "--disable-encoders",
        "--disable-decoders",
        "--disable-devices",
        "--disable-sdl2",
        "--disable-ffprobe",
        "--disable-doc",
        "--disable-w32threads",
        "--disable-ffplay",
        "--disable-avdevice",
        "--disable-swresample",
        "--disable-swscale",
        "--disable-avformat",
        "--disable-postproc",
        "--disable-avfilter",
        "--disable-pthreads",
        "--disable-os2threads",
        "--disable-network",
        "--disable-alsa",
        "--disable-bzlib",
        "--disable-zlib",
        "--disable-runtime-cpudetect",
        "--enable-decoder=h264",
        "--enable-parser=h264",
        "--enable-small",
        "--enable-static",
    ]
}
