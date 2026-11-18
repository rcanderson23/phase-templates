# Rust application example

Prepare the reusable `rust-builder` Phase with `just rust-builder` or [Build Rust Builder](../../.github/workflows/build-rust-builder.yaml).

Then run `just rust-app` or [Install Rust App](../../.github/workflows/install-rust-app.yaml). [`app.yaml`](app.yaml) defines two stages:

1. `rust-app-source` copies the application source on top of `rust-builder`.
2. `rust-app` inherits that environment, compiles and installs the binary, and saves the resulting image automatically.

There is no binary download/re-upload or local `dist/` directory. Compilation occurs in Teleport's managed build Beam. No formatting, linting, or testing is performed. Build caches and temporary output live under `/tmp` and are excluded from the saved image.

The runtime services start only when a Beam is created from `rust-app`. The example serves HTTP on `0.0.0.0:8080`; it is a demonstration, not a production HTTP implementation.

## Run the application

After the application build succeeds:

```sh
just run-rust-app
```

The recipe only creates the Beam. The Phase's `rust-app` service starts the binary; its `publish-rust-app` service waits for HTTP readiness, then runs `tsh beams publish "$BEAM_ID"` inside the Beam using its delegated identity. Both services use `context: runtime`, so neither runs during Phase builds.

Publishing happens asynchronously. To see the published URL or diagnose publication failures:

```sh
tsh beams exec <beam-name> -- beamctl logs publish-rust-app
```

The Beam remains running. When finished, remove it using the printed name:

```sh
tsh beams rm <beam-name>
```

A publishing-service failure does not remove the Beam or stop the app. Read application logs with `tsh beams exec <beam-name> -- beamctl logs rust-app`.

Reapply `app.yaml` to upload source changes and rebuild. Builder updates also trigger application rebuilds through the parent chain. Existing runtime Beams keep their original image; create a new Beam to run an updated build. Beams expire after 11 hours.

Phase parents and strategies cannot change on reapply. This example uses `rust-app`, not the previous binary-copy Phase `rust-app-release`; any old Phase remains unchanged.

To remove these application Phases, delete `rust-app` before `rust-app-source`. Keep the shared builder while other Phases depend on it.
