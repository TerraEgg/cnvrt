# Cnvrt - Image & Video File Converter

A lightweight, fast desktop application for converting image and video files across multiple formats. Built with Rust and Tauri.

## Supported Formats

### Image Formats
**Input & Output:** PNG, JPG, JPEG, BMP, GIF, TIFF, WEBP, ICO, CUR, PPM, PGM, PBM, AVIF, HEIC, EXR, DDS, TGA, FITS, DCM, PCX, PFM
**Input Only:** SVG, PDF, PSD, PSB

### Video Formats
**Input & Output:** MP4, MKV, MOV, WEBM, AVI, FLV, MPG, MPEG, TS, M2TS, MTS, OGV, OGG

## Simple install
1. Go to [releases](https://github.com/TerraEgg/cnvrt/releases)
2. Download the latest release
3. Run it

## Setup & Development

```bash
# Clone the repository
git clone https://github.com/TerraEgg/cnvrt.git
cd cnvrt

# Install dependencies
pnpm install

# Run development build
pnpm tauri dev

# Build production executable
pnpm tauri build
```

## Building FFmpeg

FFmpeg is required for video conversion and is **automatically downloaded on first video conversion**:

1. Application detects missing FFmpeg on first video conversion
2. Automatically downloads from [FFbinaries GitHub](https://github.com/ffbinaries/ffbinaries-prebuilt) (~100MB)

## Artificial Intelligence Notice
AI was used for locating & fixing bugs along side organizing certain parts of the code.

## License

MIT License - See LICENSE file for details

## Credits

Built with:
- [Tauri](https://tauri.app/) - Desktop framework
- [Vue.js](https://vuejs.org/) - Frontend framework
- [Rust](https://www.rust-lang.org/) - Backend language
- [image-rs](https://github.com/image-rs/image) - Image processing
- [FFmpeg](https://ffmpeg.org/) - Video and audio processing
