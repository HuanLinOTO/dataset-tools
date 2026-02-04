use anyhow::Result;
use std::path::Path;
use symphonia::core::audio::{AudioBufferRef, SampleBuffer, SignalSpec};
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    Wav,
    Mp3,
    Flac,
    Ogg,
}

impl AudioFormat {
    pub fn from_path(path: &Path) -> Option<Self> {
        let extension = path.extension()?.to_str()?.to_lowercase();
        match extension.as_str() {
            "wav" => Some(AudioFormat::Wav),
            "mp3" => Some(AudioFormat::Mp3),
            "flac" => Some(AudioFormat::Flac),
            "ogg" => Some(AudioFormat::Ogg),
            _ => None,
        }
    }
}

pub struct AudioFile {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub channels: usize,
}

impl AudioFile {
    /// Load an audio file from disk
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let file = std::fs::File::open(path)?;
        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        let mut hint = Hint::new();
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            hint.with_extension(ext);
        }

        let meta_opts = MetadataOptions::default();
        let fmt_opts = FormatOptions::default();

        let probed = symphonia::default::get_probe().format(&hint, mss, &fmt_opts, &meta_opts)?;
        let mut format = probed.format;

        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
            .ok_or_else(|| anyhow::anyhow!("No audio tracks found"))?;

        let dec_opts = DecoderOptions::default();
        let mut decoder = symphonia::default::get_codecs().make(&track.codec_params, &dec_opts)?;

        let track_id = track.id;
        let spec = track.codec_params.clone();
        let sample_rate = spec.sample_rate.ok_or_else(|| anyhow::anyhow!("Unknown sample rate"))? as u32;
        let channels = spec.channels.ok_or_else(|| anyhow::anyhow!("Unknown channel count"))?.count();

        let mut samples = Vec::new();

        loop {
            let packet = match format.next_packet() {
                Ok(packet) => packet,
                Err(_) => break,
            };

            if packet.track_id() != track_id {
                continue;
            }

            match decoder.decode(&packet) {
                Ok(decoded) => {
                    let mut sample_buf = Self::convert_audio_buffer(&decoded, channels)?;
                    samples.append(&mut sample_buf);
                }
                Err(_) => continue,
            }
        }

        Ok(Self {
            samples,
            sample_rate,
            channels,
        })
    }

    /// Save audio file to disk
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        let spec = hound::WavSpec {
            channels: self.channels as u16,
            sample_rate: self.sample_rate,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };

        let mut writer = hound::WavWriter::create(path, spec)?;

        for &sample in &self.samples {
            writer.write_sample(sample)?;
        }

        writer.finalize()?;
        Ok(())
    }

    fn convert_audio_buffer(buffer: &AudioBufferRef, channels: usize) -> Result<Vec<f32>> {
        let duration = buffer.frames();
        let mut sample_buf = SampleBuffer::<f32>::new(duration as u64, SignalSpec::new(buffer.spec().rate, channels.try_into()?));
        sample_buf.copy_interleaved_ref(buffer.clone());
        Ok(sample_buf.samples().to_vec())
    }

    /// Get mono version of the audio (average all channels)
    pub fn to_mono(&self) -> Vec<f32> {
        if self.channels == 1 {
            return self.samples.clone();
        }

        let mut mono = Vec::with_capacity(self.samples.len() / self.channels);
        for chunk in self.samples.chunks(self.channels) {
            let sum: f32 = chunk.iter().sum();
            mono.push(sum / self.channels as f32);
        }
        mono
    }
}
