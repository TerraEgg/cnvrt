use crate::models::ConversionError;
use std::path::Path;
use std::process::Command;
use super::formats;
use super::ffmpeg_manager;

pub struct VideoTranscoder {
    input_path: String,
    output_path: String,
    target_format: String,
    bitrate: String,
    preset: String,
}

impl VideoTranscoder {
    pub fn new(input_path: &str, output_path: &str, target_format: &str) -> Self {
        VideoTranscoder {
            input_path: input_path.to_string(),
            output_path: output_path.to_string(),
            target_format: target_format.to_lowercase(),
            bitrate: "5000k".to_string(),
            preset: "medium".to_string(),
        }
    }

    pub fn with_bitrate(mut self, bitrate: &str) -> Self {
        self.bitrate = bitrate.to_string();
        self
    }

    pub fn with_preset(mut self, preset: &str) -> Self {
        self.preset = preset.to_string();
        self
    }

    pub fn transcode(&self) -> Result<(), ConversionError> {
        let input_path = Path::new(&self.input_path);
        if !input_path.exists() {
            return Err(ConversionError::FileNotFound(
                self.input_path.clone(),
            ));
        }

        let ffmpeg_path = ffmpeg_manager::ensure_ffmpeg()
            .map_err(|e| ConversionError::ImageError(e))?;

        let codec = formats::get_video_codec_for_format(&self.target_format);
        
        let mut cmd = Command::new(ffmpeg_path);
        
        cmd.stdin(std::process::Stdio::null());
        cmd.stdout(std::process::Stdio::null());
        cmd.stderr(std::process::Stdio::null());
        
        cmd.arg("-i").arg(&self.input_path);
        
        cmd.arg("-y");
        
        cmd.arg("-c:v").arg(&codec);
        cmd.arg("-b:v").arg(&self.bitrate);
        cmd.arg("-preset").arg(&self.preset);
        
        let audio_codec = if self.target_format == "webm" {
            "libopus"
        } else {
            "aac"
        };
        cmd.arg("-c:a").arg(audio_codec);
        cmd.arg("-b:a").arg("128k");
        
        cmd.arg(&self.output_path);

        match cmd.status() {
            Ok(status) => {
                if status.success() {
                    Ok(())
                } else {
                    Err(ConversionError::ImageError(
                        "FFmpeg transcoding failed. The file format may not be supported.".to_string()
                    ))
                }
            }
            Err(e) => Err(ConversionError::ImageError(format!(
                "Failed to run FFmpeg: {}",
                e
            ))),
        }
    }
}

pub fn convert_video(
    input_path: &str,
    output_path: &str,
    target_format: &str,
) -> Result<(), ConversionError> {
    if !formats::is_video_format(target_format) {
        return Err(ConversionError::UnsupportedFormat(target_format.to_string()));
    }

    let transcoder = VideoTranscoder::new(input_path, output_path, target_format)
        .with_preset("medium");

    transcoder.transcode()
}
