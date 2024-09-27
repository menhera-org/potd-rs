# POTD-rs

Rust library to get the list of pictures from the Wikimedia Commons Picture of the Day (POTD) feed

```rust
let engine = potd::Engine::new(1920);
let picture_urls = engine.run_blocking().unwrap();
```

## License

Apache-2.0 or MPL-2.0. (Safe to use in non-copyleft projects.)
