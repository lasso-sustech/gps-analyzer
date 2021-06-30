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
cd sync_read; cargo build --release
```

**cross compiling for Raspberry Pi**:

- add `armhf` architecture by

  ```bash
  sudo nano /etc/apt/source.list.d/ubuntu-ports.list
  ```

  and paste the following content

  ```
  deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports focal main universe
  deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports focal-updates main universe
  ```

  Then run the following commands one-by-one:

  ```bash
  sudo dpkg --add-architecture armhf
  sudo apt update
  sudo apt install libudev-dev:armhf
  sudo apt install gcc-arm-linux-gnueabihf
  ```

- Run `./cross-compile.sh`, and the binary file is in `target/armv7-unknown-linux-gnueabihf/release`

### Run

```bash
cd sync_read; cargo run
```

### Test

```bash
# Run two tests at same time: read_gpio + read_gps
cargo test -- --nocapture
```

