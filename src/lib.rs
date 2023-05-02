use rand::Rng;

pub fn generate_chord_sequence(length: usize, chords: &[&str]) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut sequence = Vec::new();

    for _ in 0..length {
        let chord_index = rng.gen_range(0..chords.len());
        sequence.push(chords[chord_index].to_string());
    }

    sequence
}

