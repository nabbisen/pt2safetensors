# pt2safetensors

[![crates.io](https://img.shields.io/crates/v/pt2safetensors?label=rust)](https://crates.io/crates/pt2safetensors)
[![License](https://img.shields.io/github/license/nabbisen/pt2safetensors)](https://github.com/nabbisen/pt2safetensors/blob/main/LICENSE)
[![Documentation](https://docs.rs/pt2safetensors/badge.svg?version=latest)](https://docs.rs/pt2safetensors/latest)
[![Dependency Status](https://deps.rs/crate/pt2safetensors/latest/status.svg)](https://deps.rs/crate/pt2safetensors)

PyTorch model to safeTensors file format converter.

```rs
pt2safetensors::Pt2Safetensors::default()
    // .removes_pt_at_conversion_success()
    .convert(pt_file_path, safetensors_file_path)?;
```

---

## Open-source, with care

This project is lovingly built and maintained by volunteers. We hope it helps streamline your work. Please understand that the project has its own direction — while we welcome feedback, it might not fit every edge case 🌱

## Acknowledgements

Depends on the crates of [anyhow](https://crates.io/crates/anyhow), [candle-core](https://crates.io/crates/candle-core), [safetensors](https://crates.io/crates/safetensors).
