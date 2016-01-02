# rust-tldr

A client for [*tldr*](http://tldr-pages.github.io/), written in Rust.

*tldr* is a simplified and community-driven man pages.

## Preinstallation

### Linux

Nothing! You should be good to go!

### Windows

*Refer to the [Windows section of rust-openssl readme](https://github.com/sfackler/rust-openssl#windows)*

Or, tldr: Install OpenSSL from [here](http://slproweb.com/products/Win32OpenSSL.html). Pick 32/64-bit depending on your system. But Cargo will not be able to find OpenSSL. You can either copy the `include/openssl` directory, libssl32.dll, and libeay32.dll to locations that Cargo can find or pass the location to Cargo via environment variables:

    env OPENSSL_LIB_DIR=C:/OpenSSL-Win64 OPENSSL_INCLUDE_DIR=C:/OpenSSL-Win64/include cargo build

**Alternatively, download my 64-bit `tldr.exe` instead from https://github.com/rilut/rust-tldr/releases, and include it to your PATH.**

## Installation

```sh
$ cargo install tldr
```

## License 

MIT
