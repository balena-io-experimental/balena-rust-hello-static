# Fully static Rust binaries on balena - Hello world!

This project is based on [Rust on balena - Hello world!](https://github.com/balena-io-projects/balena-rust-hello-world)

It showcases how to create a multi-staged build process, that results in a lightweight
app image that does not have compilation tools embedded and builds a fully static binary using musl-gcc. The build target used is `arm-unknown-linux-musleabi`

## Documentation

- [`balena push` command](https://www.balena.io/docs/reference/cli/#push-applicationordevice)
- [Develop locally](https://www.balena.io/docs/learn/develop/local-mode/)
- [MUSL support for fully static binaries](https://doc.rust-lang.org/edition-guide/rust-2018/platform-and-target-support/musl-support-for-fully-static-binaries.html)
- [musl-libc](https://www.musl-libc.org/)
- [Rust Platform Support](https://forge.rust-lang.org/platform-support.html)
