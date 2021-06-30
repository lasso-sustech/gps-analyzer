# Sync Read for GPS + GPIO

### Prerequisite
- Rust Environment

  ```bash
  # download rust toolchain
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  # (after installation) setup environment
  source ~/.cargo/env

- `libudev`

  ```bash
  sudo apt install libudev-dev

### Build

```bash
cd sync_read; cargo build
```

### Run

```bash
cargo run
```

