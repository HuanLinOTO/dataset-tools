use anyhow::Result;
use audio_core::{AudioFile, AudioSlicer, SliceConfig};
use std::path::PathBuf;

/// Process an audio file and slice it into chunks
pub fn process_audio(
    input_path: &str,
    output_dir: &str,
    config: SliceConfig,
) -> Result<usize> {
    println!("Processing audio...");

    let input_path = PathBuf::from(input_path);
    let output_dir = PathBuf::from(output_dir);

    // Load audio file
    let audio = AudioFile::load(&input_path)?;
    
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

    for (i, &(start, end)) in chunks.iter().enumerate() {
        // Safely calculate indices with bounds checking
        let start_idx = start.checked_mul(audio.channels)
            .ok_or_else(|| anyhow::anyhow!("Start index overflow"))?;
        let end_idx = end.checked_mul(audio.channels)
            .ok_or_else(|| anyhow::anyhow!("End index overflow"))?;
        
        if end_idx > audio.samples.len() {
            anyhow::bail!("Slice end index {} exceeds audio length {}", end_idx, audio.samples.len());
        }
        
        let chunk_samples: Vec<f32> = audio.samples[start_idx..end_idx].to_vec();
        
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
    println!("  use audio_core::SliceConfig;");
    println!("  let config = SliceConfig::default();");
    println!("  process_audio(\"input.wav\", \"output/\", config).unwrap();");
}
