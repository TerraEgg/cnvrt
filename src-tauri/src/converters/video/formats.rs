#[derive(Debug, Clone)]
pub struct VideoFormat {
    pub extension: String,
    pub container: String,
    pub codec: String,
    pub compatible_outputs: Vec<String>,
}

pub fn get_video_format_info(format: &str) -> Option<VideoFormat> {
    match format.to_lowercase().as_str() {
        "mp4" | "m4v" => Some(VideoFormat {
            extension: "mp4".to_string(),
            container: "mp4".to_string(),
            codec: "h264".to_string(),
            compatible_outputs: vec!["mp4", "mkv", "mov", "webm", "avi", "flv", "mpg", "ts", "ogv"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        }),
        "mkv" | "matroska" => Some(VideoFormat {
            extension: "mkv".to_string(),
            container: "matroska".to_string(),
            codec: "h264".to_string(),
            compatible_outputs: vec!["mp4", "mkv", "mov", "webm", "avi", "flv", "mpg", "ts", "ogv"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        }),
        "mov" | "quicktime" => Some(VideoFormat {
            extension: "mov".to_string(),
            container: "mov".to_string(),
            codec: "h264".to_string(),
            compatible_outputs: vec!["mp4", "mkv", "mov", "webm", "avi", "flv", "mpg", "ts", "ogv"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        }),
        "webm" => Some(VideoFormat {
            extension: "webm".to_string(),
            container: "webm".to_string(),
            codec: "vp9".to_string(),
            compatible_outputs: vec!["mp4", "mkv", "mov", "avi", "flv", "mpg", "ts", "ogv", "webm"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        }),
        "avi" => Some(VideoFormat {
            extension: "avi".to_string(),
            container: "avi".to_string(),
            codec: "mpeg4".to_string(),
            compatible_outputs: vec!["mp4", "mkv", "mov", "webm", "avi", "flv", "mpg", "ts", "ogv"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        }),
        "flv" => Some(VideoFormat {
            extension: "flv".to_string(),
            container: "flv".to_string(),
            codec: "h264".to_string(),
            compatible_outputs: vec!["mp4", "mkv", "mov", "avi", "webm", "mpg", "ts", "ogv", "flv"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        }),
        "mpg" | "mpeg" | "mpeg2" => Some(VideoFormat {
            extension: "mpg".to_string(),
            container: "mpg".to_string(),
            codec: "mpeg2".to_string(),
            compatible_outputs: vec!["mp4", "mkv", "mov", "avi", "webm", "flv", "ts", "ogv", "mpg"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        }),
        "ts" | "m2ts" | "mts" => Some(VideoFormat {
            extension: "ts".to_string(),
            container: "mpegts".to_string(),
            codec: "h264".to_string(),
            compatible_outputs: vec!["mp4", "mkv", "mov", "avi", "webm", "mpg", "ogv", "ts"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        }),
        "ogv" | "ogg" => Some(VideoFormat {
            extension: "ogv".to_string(),
            container: "ogg".to_string(),
            codec: "theora".to_string(),
            compatible_outputs: vec!["mp4", "mkv", "mov", "avi", "webm", "ogv"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        }),
        _ => None,
    }
}

pub fn is_video_format(format: &str) -> bool {
    matches!(
        format.to_lowercase().as_str(),
        "mp4" | "m4v" | "mkv" | "mov" | "webm" | "avi" | "flv" | "mpg" | "mpeg" | "ts" | "m2ts" | "mts" | "ogv" | "ogg"
    )
}

pub fn get_all_video_formats() -> Vec<String> {
    vec![
        "mp4", "m4v", "mkv", "mov", "webm", "avi", "flv", "mpg", "mpeg", "ts", "m2ts", "mts", "ogv", "ogg",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}

pub fn get_video_codec_for_format(format: &str) -> String {
    match format.to_lowercase().as_str() {
        "webm" => "vp9",
        "ogv" | "ogg" => "theora",
        "ts" | "m2ts" | "mts" => "h264",
        "mpg" | "mpeg" => "mpeg2",
        "flv" => "h264",
        "avi" => "mpeg4",
        _ => "h264",
    }
    .to_string()
}
