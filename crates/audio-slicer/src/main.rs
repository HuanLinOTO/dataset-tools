use anyhow::Result;
use audio_core::{AudioFile, AudioSlicer, SliceConfig};
use std::path::PathBuf;

/// Process an audio file and slice it into chunks
pub fn process_audio(
    input_path: &str,
    output_dir: &str,
    threshold: f64,
    min_length: i64,
    min_interval: i64,
    hop_size: i64,
    max_sil_kept: i64,
) -> Result<usize> {
    println!("Processing audio...");

    let input_path = PathBuf::from(input_path);
    let output_dir = PathBuf::from(output_dir);

    // Load audio file
    let audio = AudioFile::load(&input_path)?;
    
    // Create config
    let config = SliceConfig {
        threshold,
        min_length,
        min_interval,
        hop_size,
        max_sil_kept,
    };

    // Create slicer
    let slicer = AudioSlicer::new(audio.sample_rate, config)?;
    
    // Get mono samples for slicing
    let mono_samples = audio.to_mono();
    
    // Slice the audio
    let chunks = slicer.slice(&mono_samples);

    // Save each chunk
    let stem = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("audio");

    std::fs::create_dir_all(&output_dir)?;

    for (i, (start, end)) in chunks.iter().enumerate() {
        let chunk_samples: Vec<f32> = audio.samples[*start * audio.channels..*end * audio.channels].to_vec();
        
        let chunk_audio = AudioFile {
            samples: chunk_samples,
            sample_rate: audio.sample_rate,
            channels: audio.channels,
        };

        let output_path = output_dir.join(format!("{}_{:04}.wav", stem, i));
        chunk_audio.save(&output_path)?;
    }

    println!("Successfully created {} slices", chunks.len());
    Ok(chunks.len())
}

fn main() {
    println!("Audio Slicer - Rust Edition");
    println!("===========================");
    println!();
    println!("This is a command-line version of the Audio Slicer.");
    println!("GUI support with gpui will be added in future updates.");
    println!();
    println!("Usage:");
    println!("  audio-slicer <input-file> <output-dir> [options]");
    println!();
    println!("Example:");
    println!("  audio-slicer input.wav output/");
    println!();
    println!("For now, please use this as a library in your Rust projects.");
    println!();
    println!("Example code:");
    println!("  use audio_slicer::process_audio;");
    println!("  process_audio(\"input.wav\", \"output/\", -40.0, 5000, 300, 20, 5000).unwrap();");
}
