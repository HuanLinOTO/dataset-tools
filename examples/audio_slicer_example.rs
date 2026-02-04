// Example: Using the audio-core library for audio slicing
//
// This example demonstrates how to:
// 1. Load an audio file
// 2. Configure the audio slicer
// 3. Slice the audio based on silence
// 4. Save the slices to individual files
//
// Note: This is a code example. To run it, you would need an actual audio file.

use audio_core::{AudioFile, AudioSlicer, SliceConfig};
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    println!("Audio Slicer Example");
    println!("===================\n");

    // This is an example - in practice, you'd provide actual file paths
    let input_path = "path/to/input.wav";
    let output_dir = "path/to/output/";

    println!("Step 1: Load audio file");
    println!("Loading: {}", input_path);
    
    // In a real scenario, uncomment this:
    // let audio = AudioFile::load(input_path)?;
    // println!("  Sample rate: {} Hz", audio.sample_rate);
    // println!("  Channels: {}", audio.channels);
    // println!("  Duration: {:.2} seconds", audio.samples.len() as f32 / audio.sample_rate as f32 / audio.channels as f32);

    println!("\nStep 2: Configure the slicer");
    let config = SliceConfig {
        threshold: -40.0,      // -40 dB threshold
        min_length: 5000,      // 5 seconds minimum slice length
        min_interval: 300,     // 300 ms minimum silence interval
        hop_size: 20,          // 20 ms hop size
        max_sil_kept: 5000,    // 5 seconds max silence kept
    };
    println!("  Threshold: {} dB", config.threshold);
    println!("  Min length: {} ms", config.min_length);
    println!("  Min interval: {} ms", config.min_interval);

    // In a real scenario, uncomment this:
    // println!("\nStep 3: Create slicer and slice audio");
    // let slicer = AudioSlicer::new(audio.sample_rate, config)?;
    // let mono_samples = audio.to_mono();
    // let chunks = slicer.slice(&mono_samples);
    // println!("  Found {} chunks", chunks.len());

    // println!("\nStep 4: Save chunks");
    // std::fs::create_dir_all(output_dir)?;
    // for (i, (start, end)) in chunks.iter().enumerate() {
    //     let chunk_samples: Vec<f32> = audio.samples[*start * audio.channels..*end * audio.channels].to_vec();
    //     
    //     let chunk_audio = AudioFile {
    //         samples: chunk_samples,
    //         sample_rate: audio.sample_rate,
    //         channels: audio.channels,
    //     };
    //
    //     let output_path = PathBuf::from(output_dir).join(format!("chunk_{:04}.wav", i));
    //     chunk_audio.save(&output_path)?;
    //     println!("  Saved: chunk_{:04}.wav", i);
    // }

    println!("\nExample complete!");
    println!("\nTo use this in your code:");
    println!("  1. Add audio-core to your Cargo.toml dependencies");
    println!("  2. Uncomment the actual processing code");
    println!("  3. Provide real audio file paths");
    
    Ok(())
}
