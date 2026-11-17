mod core;
pub use core::Pt2Safetensors;

#[cfg(test)]
mod tests {
    use super::*;

    /// candle-core 0.11 (T081): a path that is not a PyTorch file must
    /// be refused with an `Err`, not a panic or an empty output file -
    /// no large fixture needed, since the failure happens before any
    /// tensor data is read (the file is not even a valid zip archive).
    #[test]
    fn a_non_pytorch_file_is_refused_with_an_error_not_a_panic() {
        let dir = std::env::temp_dir().join(format!(
            "pt2safetensors-test-{}-{}",
            std::process::id(),
            "not_a_pytorch_file"
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let bad_input = dir.join("not_a_pytorch_file.bin");
        std::fs::write(&bad_input, b"this is plainly not a zip archive").unwrap();
        let output = dir.join("output.safetensors");

        let converter = Pt2Safetensors::default();
        let result = converter.convert(bad_input.clone(), output.clone());

        assert!(
            result.is_err(),
            "a non-PyTorch file must be refused, not accepted"
        );
        assert!(
            !output.exists(),
            "no output file should be written when conversion fails"
        );

        let _ = std::fs::remove_dir_all(&dir);
    }
}
