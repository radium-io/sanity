# Setup

1. Clone this repo.
1. Install required tools below for your OS.
1. Run `cargo run`
1. ???
1. Profit

## Tools

Ensure that the cargo bin directory (usually `~/.cargo/bin`) is in your PATH.
Ensure that the python bin directory is in your PATH.

### Required
- XCode (Mac OS only, must be full version not just command line tools, for using metal gfx backend)
- cmake (for building vulkan libraries)
- [rust-analyzer](https://rust-analyzer.github.io/manual.html#installation)\
  Works best with VSCode, disable official Rust plugin.

### Optional but suggested:
- [cargo-edit](https://github.com/killercup/cargo-edit) - `cargo install cargo-edit`\
  for upgrading/adding/deleting dependencies.
- [cargo-outdated](https://github.com/kbknapp/cargo-outdated) - `cargo install cargo-outdated`\
  for checking if dependencies are out of date.
- [grip](https://github.com/joeyespo/grip) - `pip install grip`\
  for editing README.md

