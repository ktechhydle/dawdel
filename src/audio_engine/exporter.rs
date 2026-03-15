use crate::interface::Track;

/// Renders all track samples + notes into a single master buffer
fn render_tracks_wav(tracks: &Vec<Track>) -> (u32, Vec<f32>) {
    if tracks.is_empty() {
        return (0, Vec::new());
    }

    // assume same sample rate across samples
    let bpm = tracks[0].bpm();
    let sample_rate = tracks[0].sample().sample_rate;
    let song_duration = tracks
        .iter()
        .map(|t| t.current_beat() * (60.0 / bpm))
        .fold(0.0, f32::max);
    let total_samples = (song_duration * (sample_rate) as f32) as usize;

    // master mix buffer
    let mut master = vec![0.0f32; total_samples];

    for track in tracks {
        let sample = track.sample();
        let sample_data = &sample.data;
        let root_note = sample.root_note;

        for note in track.notes() {
            let start_index = (note.start * (60.0 / bpm) * sample_rate as f32) as usize;
            let pitch_ratio = 2f32.powf((note.pitch as f32 - root_note as f32) / 12.0);
            let velocity = note.velocity;

            let max_samples = (note.duration * (60.0 / bpm) * sample_rate as f32) as usize;
            let mut i = 0usize;

            while i < max_samples {
                let source_index = (i as f32 * pitch_ratio) as usize;

                if source_index >= sample_data.len() {
                    break;
                }

                let dest_index = start_index + i;

                if dest_index >= master.len() {
                    break;
                }

                master[dest_index] += sample_data[source_index] * velocity;

                i += 1;
            }
        }
    }

    (sample_rate, master)
}

pub fn export_wav(name: &str, tracks: &Vec<Track>) {
    let (sample_rate, samples) = render_tracks_wav(tracks);

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: sample_rate * 2,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(format!("{}.wav", name), spec).unwrap();

    for s in samples {
        let amp = (s * i16::MAX as f32).clamp(i16::MIN as f32, i16::MAX as f32) as i16;

        writer.write_sample(amp).unwrap();
    }

    writer.finalize().unwrap();
}
