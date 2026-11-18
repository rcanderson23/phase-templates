# Rust builder

[`phase.yaml`](phase.yaml) installs native build prerequisites and uses rustup to install Rust **1.100.0** with the minimal profile: rustc, Cargo, and the native standard-library target.

The shared toolchain lives in `/opt/rustup`; rustup and Cargo commands are installed in `/usr/local/bin`. Installation leaves `/home` untouched.

Application Phases inherit this toolchain. Set `RUSTUP_HOME=/opt/rustup` and a writable `CARGO_HOME` in their build commands, as shown in the [application example](../../examples/rust-app/app.yaml). Its rebuild stage compiles and installs the binary directly into the saved application image.

[Build Rust Builder](../../.github/workflows/build-rust-builder.yaml) prepares or updates the reusable toolchain independently of [Install Rust App](../../.github/workflows/install-rust-app.yaml).

Change `--default-toolchain` to upgrade Rust. Parent updates propagate through the source copy Phase and trigger the application's rebuild stage. The Rust version is pinned; rustup, Debian packages, and Teleport's base image are not.
