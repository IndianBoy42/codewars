fn encrypt_this(text: &str) -> String {
    text.split_ascii_whitespace()
        .map(|word| {
            let mut word = word.to_string().into_bytes();
            let (first, rest) = word.split_first_mut().unwrap();
            if let Some((second, last)) = rest
                .split_first_mut()
                .and_then(|(first, rest)| rest.split_last_mut().map(|(last, _)| (first, last)))
            {
                std::mem::swap(second, last);
            }
            // first.to_string().extend(rest.iter().copied())
            first.to_string() + std::str::from_utf8(rest).unwrap()
        })
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(encrypt_this(&"A"), "65".to_string());
        assert_eq!(
            encrypt_this(&"A wise old owl lived in an oak"),
            "65 119esi 111dl 111lw 108dvei 105n 97n 111ka".to_string()
        );
        assert_eq!(
            encrypt_this(&"The more he saw the less he spoke"),
            "84eh 109ero 104e 115wa 116eh 108sse 104e 115eokp".to_string()
        );
        assert_eq!(
            encrypt_this(&"The less he spoke the more he heard"),
            "84eh 108sse 104e 115eokp 116eh 109ero 104e 104dare".to_string()
        );
        assert_eq!(
            encrypt_this(&"Why can we not all be like that wise old bird"),
            "87yh 99na 119e 110to 97ll 98e 108eki 116tah 119esi 111dl 98dri".to_string()
        );
        assert_eq!(
            encrypt_this(&"Thank you Piotr for all your help"),
            "84kanh 121uo 80roti 102ro 97ll 121ruo 104ple".to_string()
        );
    }
}
