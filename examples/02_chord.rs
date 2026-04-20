use dawdel::{ExportType, Sample, Song};
use dawdel::{chord, note};

fn main() {
    let mut song = Song::new(120.0); // 120 bpm
    let mut track1 = song.create_track(Sample::new("examples/test.wav", 60), 1); // construct a new track
    track1.chord(chord!(maj note!(C, 4)), 127, track1.current_beat(), 2.0);

    song.add_track(track1);
    song.export("output.wav", ExportType::WAV(44100), true); // export the wav audio at 44100 samples and open it
}
