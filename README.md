# Phase templates

Reusable [Teleport Phases](https://goteleport.com/docs/beams/) for preparing applications to run in Beams.

```text
rust-builder                  reusable toolchain
  └── rust-app-source         application source
        └── rust-app          compiled binary + runtime service
```

## Build locally

With `just` and an authenticated `tsh` installed:

```sh
just rust-builder  # Build the Rust builder Phase
just rust-app      # Build the application Phase using the existing builder
just run-rust-app  # Start a Beam; its services start and publish the app
```

These commands use your local Teleport identity, not the GitHub bot.

## Build with GitHub Actions

- [Build Rust Builder](.github/workflows/build-rust-builder.yaml) creates or updates the toolchain Phase.
- [Install Rust App](.github/workflows/install-rust-app.yaml) applies the source and application Phases on top of the existing builder. Teleport compiles and saves the application directly; no binary download/re-upload is needed.

Both jobs require the **`beams`** GitHub environment. In **Settings → Environments**, create it, allow deployments only from `main`, and configure required reviewers (where supported by your GitHub plan). The workflow declaration alone does not configure those protections.

With administrator credentials, apply the [bot](teleport/bot.yaml) and [GitHub join token](teleport/github-token.yaml):

```sh
tctl create --force teleport/bot.yaml
tctl create --force teleport/github-token.yaml
```

The bot assumes `beam-user`. The token requires this environment, repository, branch, and workflow name. These commands replace existing configurations; review them before applying.
