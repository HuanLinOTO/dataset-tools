use anyhow::Result;
use audio_core::{AudioFile, AudioSlicer, SliceConfig};
use gpui::*;
use std::path::PathBuf;
use std::sync::Arc;

struct AudioSlicerApp {
    input_path: String,
    output_dir: String,
    threshold: f64,
    min_length: i64,
    min_interval: i64,
    hop_size: i64,
    max_sil_kept: i64,
    status: String,
}

impl AudioSlicerApp {
    fn new() -> Self {
        Self {
            input_path: String::new(),
            output_dir: String::new(),
            threshold: -40.0,
            min_length: 5000,
            min_interval: 300,
            hop_size: 20,
            max_sil_kept: 5000,
            status: "Ready".to_string(),
        }
    }

    fn process_audio(&mut self) -> Result<()> {
        if self.input_path.is_empty() {
            self.status = "Error: No input file selected".to_string();
            return Ok(());
        }

        if self.output_dir.is_empty() {
            self.status = "Error: No output directory selected".to_string();
            return Ok(());
        }

        self.status = "Processing...".to_string();

        let input_path = PathBuf::from(&self.input_path);
        let output_dir = PathBuf::from(&self.output_dir);

        // Load audio file
        let audio = AudioFile::load(&input_path)?;
        
        // Create config
        let config = SliceConfig {
            threshold: self.threshold,
            min_length: self.min_length,
            min_interval: self.min_interval,
            hop_size: self.hop_size,
            max_sil_kept: self.max_sil_kept,
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

        self.status = format!("Successfully created {} slices", chunks.len());
        Ok(())
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| AudioSlicerApp::new())
        })
        .unwrap();
    });
}

impl Render for AudioSlicerApp {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x2e2e2e))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p_4()
                    .gap_4()
                    .child(
                        div()
                            .text_xl()
                            .text_color(rgb(0xffffff))
                            .child("Audio Slicer")
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                div()
                                    .text_color(rgb(0xcccccc))
                                    .child("Input File:")
                            )
                            .child(
                                div()
                                    .text_color(rgb(0xffffff))
                                    .child(if self.input_path.is_empty() {
                                        "None selected"
                                    } else {
                                        &self.input_path
                                    })
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                div()
                                    .text_color(rgb(0xcccccc))
                                    .child("Output Directory:")
                            )
                            .child(
                                div()
                                    .text_color(rgb(0xffffff))
                                    .child(if self.output_dir.is_empty() {
                                        "None selected"
                                    } else {
                                        &self.output_dir
                                    })
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                div()
                                    .text_color(rgb(0xcccccc))
                                    .child(format!("Threshold: {:.1} dB", self.threshold))
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                div()
                                    .text_color(rgb(0xcccccc))
                                    .child(format!("Min Length: {} ms", self.min_length))
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                div()
                                    .text_color(rgb(0xcccccc))
                                    .child(format!("Min Interval: {} ms", self.min_interval))
                            )
                    )
                    .child(
                        div()
                            .p_4()
                            .text_color(rgb(0xffffff))
                            .bg(rgb(0x404040))
                            .child(&self.status)
                    )
            )
    }
}
