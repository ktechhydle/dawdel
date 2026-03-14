/// A sample object based on a `.wav` file and root note representing the pitch shift of the sound sample.
#[derive(Debug, Clone)]
pub struct Sample {
    name: String,
    wav_path: String,
    root_note: u8,
}

impl Sample {
    /// Constructs a new sample object, where `name` is a string of the sample name,
    /// `wav_path` is the path to the WAV file, and `root_note` is the midi note number (0-127)
    /// the sample is pitch shifted by.
    ///
    /// # Examples
    ///
    /// ```
    /// let sample = Sample::new("piano", "my_samples/piano.wav", 60); // C4 = root note, anything above or below will be pitch shifted
    /// ```
    pub fn new(name: &str, wav_path: &str, root_note: u8) -> Self {
        Self {
            name: name.to_string(),
            wav_path: wav_path.to_string(),
            root_note,
        }
    }
}
