/// Trait for audio effect implementations.
///
/// `modify` takes the input `data` (left and right audio channels) and outputs `Vec<(f32, f32)>`
/// (left and right audio channels). Anything that happens between is described as an "audio effect".
///
/// # Example
///
/// ```
/// pub struct VolumeEffect {
///     amount: f32,
/// }
///
/// impl Effect for VolumeEffect {
///     fn modify(&self, sample_rate: u32, data: &[(f32, f32)]) -> Vec<(f32, f32)> {
///         let new_data = Vec::new();
///
///         for (l, r) in data {
///             new_data.push((l + self.amount, r + self.amount))
///         }
///
///         new_data
///     }
/// }
/// ```
pub trait Effect {
    fn modify(&self, sample_rate: u32, data: &[(f32, f32)]) -> Vec<(f32, f32)>;
}

/// A reverb effect for audio samples.
pub struct ReverbEffect {
    room_size: f32,
    wet: f32,
    dry: f32,
}

impl ReverbEffect {
    /// Constructs a new reverb effect, where `room_size` represents the simulated "room", `wet` is the wet amount, and `dry` is the dry amount.
    pub fn new(room_size: f32, wet: f32, dry: f32) -> Self {
        Self {
            room_size,
            wet,
            dry,
        }
    }
}

impl Effect for ReverbEffect {
    fn modify(&self, sample_rate: u32, data: &[(f32, f32)]) -> Vec<(f32, f32)> {
        let delay_samples = (sample_rate as f32 * self.room_size) as usize;
        let mut output = vec![(0.0, 0.0); data.len()];

        for i in 0..data.len() {
            let (l, r) = data[i];

            let delayed = if i >= delay_samples {
                output[i - delay_samples]
            } else {
                (0.0, 0.0)
            };

            let out_l = l * self.dry + delayed.0 * self.wet;
            let out_r = r * self.dry + delayed.1 * self.wet;

            output[i] = (out_l, out_r);
        }

        output
    }
}
