default:
    @just --list

# Create or update the reusable Rust toolchain Phase.
rust-builder:
    tsh phase apply -f templates/rust/phase.yaml

# Build the application Phase on top of the existing builder.
rust-app:
    tsh phase apply -f examples/rust-app/app.yaml

# Start a Beam; its runtime services start and publish the app automatically.
run-rust-app:
    tsh beams add --phase=rust-app --no-console
