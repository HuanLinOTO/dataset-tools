use anyhow::Result;
use rubato::{FftFixedIn, Resampler as RubatoResampler};

pub struct Resampler {
    input_rate: u32,
    output_rate: u32,
}

impl Resampler {
    pub fn new(input_rate: u32, output_rate: u32) -> Self {
        Self {
            input_rate,
            output_rate,
        }
    }

    pub fn resample(&self, input: &[f32], channels: usize) -> Result<Vec<f32>> {
        if self.input_rate == self.output_rate {
            return Ok(input.to_vec());
        }

        const CHUNK_SIZE: usize = 1024;
        // Sub-chunks parameter: controls internal chunking for better cache locality
        const SUB_CHUNKS: usize = 2;
        
        let mut resampler = FftFixedIn::<f32>::new(
            self.input_rate as usize,
            self.output_rate as usize,
            CHUNK_SIZE,
            SUB_CHUNKS,
            channels,
        )?;

        // Deinterleave input
        let frames = input.len() / channels;
        let mut channel_data: Vec<Vec<f32>> = vec![Vec::with_capacity(frames); channels];
        
        for (i, sample) in input.iter().enumerate() {
            channel_data[i % channels].push(*sample);
        }

        // Process in chunks
        let mut output_channels: Vec<Vec<f32>> = vec![Vec::new(); channels];
        
        for chunk_start in (0..frames).step_by(CHUNK_SIZE) {
            let chunk_end = (chunk_start + CHUNK_SIZE).min(frames);
            let mut chunk: Vec<Vec<f32>> = channel_data
                .iter()
                .map(|ch| ch[chunk_start..chunk_end].to_vec())
                .collect();

            // Pad the last chunk if needed
            if chunk[0].len() < CHUNK_SIZE {
                for ch in &mut chunk {
                    ch.resize(CHUNK_SIZE, 0.0);
                }
            }

            match resampler.process(&chunk, None) {
                Ok(output) => {
                    for (i, ch) in output.iter().enumerate() {
                        output_channels[i].extend_from_slice(ch);
                    }
                }
                Err(e) => return Err(anyhow::anyhow!("Resampling error: {:?}", e)),
            }
        }

        // Interleave output
        let output_frames = output_channels[0].len();
        let mut result = Vec::with_capacity(output_frames * channels);
        
        for i in 0..output_frames {
            for ch in &output_channels {
                if i < ch.len() {
                    result.push(ch[i]);
                }
            }
        }

        Ok(result)
    }
}
