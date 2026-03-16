/// Converts a note name into the corresponding midi note number.
///
/// # Example
///
/// ```
/// use dawdel::note;
///
/// let note_num = note!(C, 4);
/// assert_eq!(note_num, 60)
/// ```
#[macro_export]
macro_rules! note {
    (C, $oct:literal) => {
        (($oct + 1) * 12 + 0)
    };
    (Cs, $oct:literal) => {
        (($oct + 1) * 12 + 1)
    };
    (D, $oct:literal) => {
        (($oct + 1) * 12 + 2)
    };
    (Ds, $oct:literal) => {
        (($oct + 1) * 12 + 3)
    };
    (E, $oct:literal) => {
        (($oct + 1) * 12 + 4)
    };
    (F, $oct:literal) => {
        (($oct + 1) * 12 + 5)
    };
    (Fs, $oct:literal) => {
        (($oct + 1) * 12 + 6)
    };
    (G, $oct:literal) => {
        (($oct + 1) * 12 + 7)
    };
    (Gs, $oct:literal) => {
        (($oct + 1) * 12 + 8)
    };
    (A, $oct:literal) => {
        (($oct + 1) * 12 + 9)
    };
    (As, $oct:literal) => {
        (($oct + 1) * 12 + 10)
    };
    (B, $oct:literal) => {
        (($oct + 1) * 12 + 11)
    };
}

/// Converts a chord name into the corresponding midi note numbers.
///
/// # Example
///
/// ```
/// use dawdel::{chord, note};
///
/// let note_nums = chord!(maj note!(C, 4));
/// assert_eq!(note_nums, vec![60, 64, 67])
/// ```
#[macro_export]
macro_rules! chord {
    (maj $root:expr) => {
        vec![$root, $root + 4, $root + 7]
    };

    (min $root:expr) => {
        vec![$root, $root + 3, $root + 7]
    };

    (maj7 $root:expr) => {
        vec![$root, $root + 4, $root + 7, $root + 11]
    };

    (min7 $root:expr) => {
        vec![$root, $root + 3, $root + 7, $root + 10]
    };
}

/// A note object, containing pitch (0-127), start time, duration, and velocity.
#[derive(Debug, Clone)]
pub struct Note {
    pub pitch: u8,
    pub velocity: f32,
    pub start: f32,
    pub duration: f32,
}
