use anyhow::Result;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct SliceConfig {
    /// Threshold in dB (e.g., -40.0)
    pub threshold: f64,
    /// Minimum length of a slice in milliseconds
    pub min_length: i64,
    /// Minimum interval between slices in milliseconds
    pub min_interval: i64,
    /// Hop size in milliseconds
    pub hop_size: i64,
    /// Maximum silence kept at edges in milliseconds
    pub max_sil_kept: i64,
}

impl Default for SliceConfig {
    fn default() -> Self {
        Self {
            threshold: -40.0,
            min_length: 5000,
            min_interval: 300,
            hop_size: 20,
            max_sil_kept: 5000,
        }
    }
}

pub struct AudioSlicer {
    _config: SliceConfig,
    _sample_rate: u32,
    threshold_linear: f64,
    hop_size_samples: i64,
    win_size_samples: i64,
    min_length_hops: i64,
    min_interval_hops: i64,
    max_sil_kept_hops: i64,
}

impl AudioSlicer {
    pub fn new(sample_rate: u32, config: SliceConfig) -> Result<Self> {
        // Validate config
        if !(config.min_length >= config.min_interval && config.min_interval >= config.hop_size) {
            anyhow::bail!(
                "Invalid config: min_length >= min_interval >= hop_size must be satisfied"
            );
        }
        if config.max_sil_kept < config.hop_size {
            anyhow::bail!("Invalid config: max_sil_kept >= hop_size must be satisfied");
        }

        let threshold_linear = 10f64.powf(config.threshold / 20.0);
        let hop_size_samples = (config.hop_size * sample_rate as i64 + 500) / 1000;
        let win_size_samples = std::cmp::min(
            (config.min_interval * sample_rate as i64 + 500) / 1000,
            4 * hop_size_samples,
        );
        let min_length_hops =
            (config.min_length * sample_rate as i64 + 500) / (1000 * hop_size_samples);
        let min_interval_hops =
            (config.min_interval * sample_rate as i64 + 500) / (1000 * hop_size_samples);
        let max_sil_kept_hops =
            (config.max_sil_kept * sample_rate as i64 + 500) / (1000 * hop_size_samples);

        Ok(Self {
            _config: config,
            _sample_rate: sample_rate,
            threshold_linear,
            hop_size_samples,
            win_size_samples,
            min_length_hops,
            min_interval_hops,
            max_sil_kept_hops,
        })
    }

    /// Slice audio into chunks based on silence detection
    /// Returns a list of (start_frame, end_frame) tuples
    pub fn slice(&self, samples: &[f32]) -> Vec<(usize, usize)> {
        let frames = samples.len();

        // If audio is too short, return the whole thing
        if (frames as i64 + self.hop_size_samples - 1) / self.hop_size_samples <= self.min_length_hops
        {
            return vec![(0, frames)];
        }

        // Calculate RMS for each hop
        let rms_list = self.calculate_rms(samples);

        // Find silence tags
        let sil_tags = self.find_silence_tags(&rms_list);

        // Convert silence tags to chunks
        self.tags_to_chunks(&sil_tags, frames, &rms_list)
    }

    fn calculate_rms(&self, samples: &[f32]) -> Vec<f64> {
        let frames = samples.len();
        let rms_size = (frames as i64 / self.hop_size_samples + 1) as usize;
        let mut rms_list = vec![0.0; rms_size];

        let mut moving_rms = MovingRMS::new(self.win_size_samples as usize);
        let padding = (self.win_size_samples / 2) as usize;

        // Add initial padding
        for _ in 0..padding {
            moving_rms.push(0.0);
        }

        // Process initial samples
        let initial_end = std::cmp::min(padding, frames);
        for i in 0..initial_end {
            moving_rms.push(samples[i] as f64);
        }

        rms_list[0] = moving_rms.rms();
        let mut rms_index = 1;

        // Process rest of samples in hops
        let hop_size = self.hop_size_samples as usize;
        let mut pos = padding;

        while pos < frames && rms_index < rms_size {
            let chunk_end = std::cmp::min(pos + hop_size, frames);
            for i in pos..chunk_end {
                moving_rms.push(samples[i] as f64);
            }
            // Pad if needed
            for _ in chunk_end..(pos + hop_size) {
                moving_rms.push(0.0);
            }
            rms_list[rms_index] = moving_rms.rms();
            rms_index += 1;
            pos += hop_size;
        }

        // Fill remaining with padding
        while rms_index < rms_size {
            for _ in 0..hop_size {
                moving_rms.push(0.0);
            }
            rms_list[rms_index] = moving_rms.rms();
            rms_index += 1;
        }

        rms_list
    }

    fn find_silence_tags(&self, rms_list: &[f64]) -> Vec<(i64, i64)> {
        let mut sil_tags = Vec::new();
        let mut silence_start = 0i64;
        let mut has_silence_start = false;
        let mut clip_start = 0i64;

        for i in 0..rms_list.len() {
            let rms = rms_list[i];
            let i = i as i64;

            if rms < self.threshold_linear {
                if !has_silence_start {
                    silence_start = i;
                    has_silence_start = true;
                }
                continue;
            }

            if !has_silence_start {
                continue;
            }

            let is_leading_silence = silence_start == 0 && i > self.max_sil_kept_hops;
            let need_slice_middle = (i - silence_start) >= self.min_interval_hops
                && (i - clip_start) >= self.min_length_hops;

            if !is_leading_silence && !need_slice_middle {
                has_silence_start = false;
                continue;
            }

            // Need slicing
            if i - silence_start <= self.max_sil_kept_hops {
                let pos = argmin_range(rms_list, silence_start as usize, (i + 1) as usize)
                    as i64
                    + silence_start;
                if silence_start == 0 {
                    sil_tags.push((0, pos));
                } else {
                    sil_tags.push((pos, pos));
                }
                clip_start = pos;
            } else if i - silence_start <= self.max_sil_kept_hops * 2 {
                let pos = argmin_range(
                    rms_list,
                    (i - self.max_sil_kept_hops) as usize,
                    (silence_start + self.max_sil_kept_hops + 1) as usize,
                ) as i64
                    + i
                    - self.max_sil_kept_hops;
                let pos_l = argmin_range(
                    rms_list,
                    silence_start as usize,
                    (silence_start + self.max_sil_kept_hops + 1) as usize,
                ) as i64
                    + silence_start;
                let pos_r = argmin_range(
                    rms_list,
                    (i - self.max_sil_kept_hops) as usize,
                    (i + 1) as usize,
                ) as i64
                    + i
                    - self.max_sil_kept_hops;

                if silence_start == 0 {
                    clip_start = pos_r;
                    sil_tags.push((0, clip_start));
                } else {
                    clip_start = std::cmp::max(pos_r, pos);
                    sil_tags.push((std::cmp::min(pos_l, pos), clip_start));
                }
            } else {
                let pos_l = argmin_range(
                    rms_list,
                    silence_start as usize,
                    (silence_start + self.max_sil_kept_hops + 1) as usize,
                ) as i64
                    + silence_start;
                let pos_r = argmin_range(
                    rms_list,
                    (i - self.max_sil_kept_hops) as usize,
                    (i + 1) as usize,
                ) as i64
                    + i
                    - self.max_sil_kept_hops;

                if silence_start == 0 {
                    sil_tags.push((0, pos_r));
                } else {
                    sil_tags.push((pos_l, pos_r));
                }
                clip_start = pos_r;
            }
            has_silence_start = false;
        }

        // Deal with trailing silence
        let total_frames = rms_list.len() as i64;
        if has_silence_start && (total_frames - silence_start) >= self.min_interval_hops {
            let silence_end = std::cmp::min(total_frames - 1, silence_start + self.max_sil_kept_hops);
            let pos =
                argmin_range(rms_list, silence_start as usize, (silence_end + 1) as usize) as i64
                    + silence_start;
            sil_tags.push((pos, total_frames + 1));
        }

        sil_tags
    }

    fn tags_to_chunks(
        &self,
        sil_tags: &[(i64, i64)],
        total_frames: usize,
        rms_list: &[f64],
    ) -> Vec<(usize, usize)> {
        if sil_tags.is_empty() {
            return vec![(0, total_frames)];
        }

        let mut chunks = Vec::new();
        let total_rms_frames = rms_list.len() as i64;

        // Add chunk before first silence
        let s0 = sil_tags[0].0;
        if s0 > 0 {
            let end = std::cmp::min(
                total_frames,
                (s0 * self.hop_size_samples) as usize,
            );
            chunks.push((0, end));
        }

        // Add chunks between silences
        for i in 0..sil_tags.len() - 1 {
            let begin = (sil_tags[i].1 * self.hop_size_samples) as usize;
            let end = std::cmp::min(
                total_frames,
                (sil_tags[i + 1].0 * self.hop_size_samples) as usize,
            );
            chunks.push((begin, end));
        }

        // Add chunk after last silence
        if sil_tags.last().unwrap().1 < total_rms_frames {
            let begin = (sil_tags.last().unwrap().1 * self.hop_size_samples) as usize;
            chunks.push((begin, total_frames));
        }

        chunks
    }
}

struct MovingRMS {
    window_size: usize,
    square_sum: f64,
    queue: VecDeque<f64>,
}

impl MovingRMS {
    fn new(window_size: usize) -> Self {
        Self {
            window_size,
            square_sum: 0.0,
            queue: VecDeque::new(),
        }
    }

    fn push(&mut self, num: f64) {
        let num_squared = num * num;
        
        if self.queue.len() < self.window_size {
            self.queue.push_back(num_squared);
            self.square_sum += num_squared;
        } else {
            let front_item = self.queue.pop_front().unwrap_or(0.0);
            self.queue.push_back(num_squared);
            self.square_sum += num_squared - front_item;
        }
    }

    fn rms(&self) -> f64 {
        if self.window_size == 0 || self.square_sum < 0.0 {
            return 0.0;
        }
        (self.square_sum.max(0.0) / self.window_size as f64).sqrt()
    }
}

fn argmin_range(v: &[f64], begin: usize, end: usize) -> usize {
    let size = v.len();
    let begin = begin.min(size);
    let end = end.min(size);
    
    if begin >= end {
        return 0;
    }

    let slice = &v[begin..end];
    slice
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(i, _)| i)
        .unwrap_or(0)
}
