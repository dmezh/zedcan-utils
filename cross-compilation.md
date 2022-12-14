## Cross-compilation help for macOS + brew for cargo

You need a cross-toolchain for `armv7-unknown-linux-gnueabihf`.

### Getting the toolchain

```bash
brew tap messense/macos-cross-toolchain
brew install armv7-unknown-linux-gnueabihf
```

### Rust setup
```bash
rustup target add armv7-unknown-linux-gnueabihf
```

Add this to your `~/.cargo/config`, use version from `brew install` (`11.2.0` in below example):
```toml
[target.armv7-unknown-linux-gnueabihf]
linker = "/opt/homebrew/Cellar/armv7-unknown-linux-gnueabihf/11.2.0/toolchain/bin/armv7-unknown-linux-gnueabihf-gcc"
```

### Building stuff with cargo
cargo build --target=armv7-unknown-linux-gnueabihf

### You should get ready-to-use binaries! Not too bad.
