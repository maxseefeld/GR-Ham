use gr_ham::morse::MorseEncoder;
use gr_ham::callsign::Callsign;
use gr_ham::util::freq::Freq;

fn main() {
    // Set up the Morse encoder
    let morse = MorseEncoder::new();

    // Set up the callsign
    let callsign = Callsign::parse("N0CALL").unwrap();

    // Set up the frequency
    let freq = Freq::from_mhz(14.1).unwrap();

    // Encode the message in Morse code
    let message = "CQ CQ DE N0CALL".to_string();
    let encoded = morse.encode(&message).unwrap();

    // Send the message on the specified frequency
    let _ = gr_ham::send(&callsign, &encoded, &freq);
}
