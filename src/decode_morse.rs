struct MorseDecoder;

impl MorseDecoder {
    fn new() -> MorseDecoder {
        MorseDecoder
    }
}

impl MorseDecoder {
    pub fn decode_bits(&self, encoded: &str) -> String {
        // encoded.bytes().copied();

        unimplemented!()
    }
    
    pub fn decode_morse(&self, encoded: &str) -> String {
        unimplemented!()
        // call the preloaded hash with: self.morse_code.get(...)
    }
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn examples() {
    let decoder = MorseDecoder::new();
    assert_eq!(decoder.decode_morse(&decoder.decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011")), "HEY JUDE".to_string());
}