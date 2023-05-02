use week6_ds581::generate_chord_sequence;

fn main() {
    let chords = ["C", "D", "E", "F", "G", "A", "B", "Am", "Dm", "Em", "Fmaj7", "G7"];
    let sequence = generate_chord_sequence(4, &chords);
    println!("{:?}", sequence);
}
