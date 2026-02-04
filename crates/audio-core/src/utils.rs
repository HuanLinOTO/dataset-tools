/// Utility functions for audio processing

/// Convert decibels to linear scale
pub fn db_to_linear(db: f64) -> f64 {
    10f64.powf(db / 20.0)
}

/// Convert linear scale to decibels
pub fn linear_to_db(linear: f64) -> f64 {
    20.0 * linear.log10()
}

/// Normalize audio samples to range [-1.0, 1.0]
pub fn normalize(samples: &mut [f32]) {
    let max_val = samples
        .iter()
        .map(|s| s.abs())
        .fold(0.0f32, |a, b| a.max(b));
    
    if max_val > 0.0 {
        for sample in samples.iter_mut() {
            *sample /= max_val;
        }
    }
}

/// Calculate RMS (Root Mean Square) energy of audio samples
pub fn calculate_rms(samples: &[f32]) -> f64 {
    if samples.is_empty() {
        return 0.0;
    }
    
    let sum_squares: f64 = samples.iter().map(|&s| (s as f64).powi(2)).sum();
    (sum_squares / samples.len() as f64).sqrt()
}
