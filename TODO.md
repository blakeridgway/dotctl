# dotctl TODO

## Core MVP

- [x] Parse `dotfiles.toml` manifest
- [x] Implement `dotctl sync`
	- symlink entries
	- copy file entires
	- render template entries
	- backup existing targets
- [x] Implement `dotctl bootstrap`
	- package install (apt, brew, flatpak)
	- simple `run_once` scripts
- [x] Add basic CLI with `clap`/`structopt`
- [ ] Write unit tests for manifests parsing and sync logic

## QoL Updates

- [ ] `--dry-run` mode for sync & bootstrap
- [ ] `dotctl status` \ `dotctl diff`
- [ ] Progress reporting with `indicatif`
- [ ] Error aggregation & pretty-printing
- [ ] Shell completion scripts

## Cross-Platform & Profiles

- [ ] Platform detection & per-platform installs
- [ ] Support multiple profiles (e.g. `dotfiles.laptop.toml, dotfiles.desktop.toml`)
- [ ] Merge base + host overrides

## Advanced Features

- [ ] Template context variables (env, user, hostname)
- [ ] Interactive prompts with `dialoguer` for any conflicts
- [ ] Remote deploy via `ssh` or `ssh2` crate
- [ ] Self-update command
- [ ] TUI dashboard with `tui-rs`

## Documentation & CI

- [ ] Auto-generate README/manifest docs
- [ ] Add GitHub Actions to run `dotctl diff` on PRs
- [ ] Publish crate to crates.io / GitHub releases
