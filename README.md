1. Clone
2. Run `rustup default nightly` or comment out `cargo-features = ["profile-rustflags"]` in the cargo.toml
3. Make environment files
    Windows: .env_test.bat and .env_prod.bat, using the contents of the samples
            Run `.env_test.bat` to load variables
    Linux: TODO