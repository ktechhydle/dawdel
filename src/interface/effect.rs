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
        let reflections = [
            (0.012, 0.6),
            (0.017, 0.5),
            (0.023, 0.4),
            (0.031, 0.3),
            (0.045, 0.2),
        ];

        let mut output = data.to_vec();

        for &(delay_sec, decay) in &reflections {
            let delay_samples = (delay_sec * self.room_size * sample_rate as f32) as usize;

            for i in delay_samples..output.len() {
                let (dl, dr) = output[i - delay_samples];

                output[i].0 += dl * decay * self.wet;
                output[i].1 += dr * decay * self.wet;
            }
        }

        for i in 0..output.len() {
            output[i].0 = output[i].0 * self.wet + data[i].0 * self.dry;

            output[i].1 = output[i].1 * self.wet + data[i].1 * self.dry;
        }

        output
    }
}
