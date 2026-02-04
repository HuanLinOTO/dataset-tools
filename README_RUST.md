# Dataset Tools (Rust + gpui)

DiffSinger dataset processing tools refactored in Rust with planned gpui GUI framework.

## Current Status

✅ **Core Audio Processing Library** - Fully implemented in Rust
- Audio file I/O (WAV, MP3, FLAC, OGG)
- High-quality resampling
- Silence-based audio slicing
- Audio utilities

🚧 **GUI Applications** - Currently CLI placeholders, gpui GUI implementation planned
- AudioSlicer - Audio slicing functionality available as library
- MinLabel - To be implemented
- SlurCutter - To be implemented
- LyricFA - To be implemented
- HubertFA - To be implemented
- SomeInfer - To be implemented

## Applications

All applications now use pure Rust. GUI with gpui framework is planned for future releases:

+ **AudioSlicer** - Slice audio files based on silence detection (library ready)
+ **MinLabel** - Audio labeling and annotation tool (planned)
+ **SlurCutter** - Detect and cut audio slurs (planned)
+ **LyricFA** - Lyric forced alignment with ASR (planned)
+ **HubertFA** - Forced alignment using Hubert models (planned)
+ **SomeInfer** - Model inference tool (planned)

## Features

- ✅ Pure Rust implementation for safety and performance
- ✅ Modern audio processing with Symphonia and rubato
- ✅ Clean workspace architecture with shared core library
- ✅ Type-safe and memory-safe by default
- 🚧 GUI with gpui framework (planned for future releases)

## Supported Platforms

+ Microsoft Windows (10+)
+ Apple macOS (11+)
+ Linux (Ubuntu 20.04+)

## Requirements

### Build Requirements

| Component | Requirement |           Notes           |
|:---------:|:-----------:|:-------------------------:|
|   Rust    |   >=1.75    |   Stable toolchain        |
|   Cargo   |   Latest    |   Comes with Rust         |
|   CMake   |   >=3.17    |   For native dependencies |

### Runtime Requirements

- Standard Rust runtime (no additional dependencies for CLI tools)
- GPU support for GUI (when implemented) will require Vulkan, Metal, or DirectX 12

## Building from Source

### Install Rust

```sh
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Or on Windows, download from https://rustup.rs
```

### Clone and Build

```sh
git clone https://github.com/HuanLinOTO/dataset-tools.git
cd dataset-tools

# Build all applications in release mode
cargo build --release

# Or build a specific application
cargo build --release -p audio-slicer
cargo build --release -p min-label
cargo build --release -p slur-cutter
cargo build --release -p lyric-fa
cargo build --release -p hubert-fa
cargo build --release -p some-infer
```

### Run Applications

```sh
# Run from source
cargo run --release -p audio-slicer

# Or run the compiled binary
./target/release/audio-slicer
```

## Project Structure

```
dataset-tools/
├── Cargo.toml                 # Workspace configuration
├── crates/
│   ├── audio-core/           # Shared audio processing library
│   │   ├── src/
│   │   │   ├── audio_file.rs # Audio I/O
│   │   │   ├── resampler.rs  # Sample rate conversion
│   │   │   ├── slicer.rs     # Silence-based slicing
│   │   │   └── utils.rs      # Utility functions
│   │   └── Cargo.toml
│   ├── audio-slicer/         # Audio Slicer application
│   ├── min-label/            # MinLabel application
│   ├── slur-cutter/          # SlurCutter application
│   ├── lyric-fa/             # LyricFA application
│   ├── hubert-fa/            # HubertFA application
│   └── some-infer/           # SomeInfer application
└── README_RUST.md            # This file
```

## Core Library (audio-core)

The `audio-core` crate provides shared functionality:

### Audio File I/O
- Load: WAV, MP3, FLAC, OGG (via Symphonia)
- Save: WAV (via hound)
- Multi-channel support
- Automatic format detection

### Audio Processing
- High-quality resampling (via rubato FFT-based resampler)
- RMS calculation for silence detection
- Mono conversion from multi-channel audio
- Audio normalization

### Audio Slicing
- Intelligent silence-based slicing
- Configurable parameters:
  - Threshold (dB)
  - Minimum slice length
  - Minimum interval between slices
  - Maximum silence kept

## Dependencies

### Core Libraries

+ [Symphonia](https://github.com/pdm-project/symphonia) - Pure Rust audio decoding
  - MPL v2.0
+ [rubato](https://github.com/HEnquist/rubato) - High-quality audio resampling
  - MIT License
+ [hound](https://github.com/ruuda/hound) - WAV encoding/decoding
  - Apache 2.0
+ [rustfft](https://github.com/PolySync/rust-fft) - Fast Fourier Transform
  - MIT/Apache 2.0

### Planned Libraries

+ [gpui](https://github.com/zed-industries/zed) - Modern GPU-accelerated UI framework (planned)
  - Apache 2.0 / GPL v3.0

## Migration from C++/Qt

This Rust rewrite brings several advantages:

1. **Memory Safety**: Rust's ownership system prevents common C++ bugs
2. **Modern Architecture**: Clean separation of core library and applications
3. **Easier Deployment**: Single binary with fewer runtime dependencies
4. **Better Tooling**: Cargo makes building and dependency management simple
5. **Cross-platform**: Same codebase for all platforms

### Current Progress

✅ Core audio processing library fully implemented in Rust
- Audio I/O for multiple formats
- High-quality resampling
- Silence-based slicing algorithm (ported from C++)
- Audio utilities

🚧 GUI applications with gpui
- Framework selected (gpui from Zed)
- Implementation planned for future release
- CLI tools available now for immediate use

## Development

### Running Tests

```sh
cargo test
```

### Code Formatting

```sh
cargo fmt
```

### Linting

```sh
cargo clippy
```

## License

This repository is licensed under the Apache 2.0 License.

## Related Projects

+ [DiffSinger](https://github.com/openvpi/DiffSinger) - Singing voice synthesis
  - Apache 2.0 License

## Migration Notes

The original C++/Qt codebase has been completely rewritten in Rust. Key changes:

- Qt → gpui for GUI framework
- C++ → Rust for all core logic
- CMake/vcpkg → Cargo for build system
- libsndfile → Symphonia for audio I/O
- r8brain → rubato for resampling

All core functionality has been preserved while improving safety, performance, and maintainability.
