# Moonsanity

This game is an exploration of three things:

1. Rust
1. [Amethyst](https://amethyst.rs/)
1. Spooky Moon Dungeons

I was able to combine Amethyst with the wonderful [WaveFunctionCollapse](https://github.com/mxgmn/WaveFunctionCollapse) [implementation by @stevebob](https://github.com/stevebob/wfc)

Additional thanks to [Elthen](https://www.patreon.com/elthen) for the [wonderful free sprite sets available here](https://elthen.itch.io/2d-pixel-art-dungeon-tileset), and [Adam Saltsman's Sci Fi Sprites](https://adamatomic.itch.io/sci-fi-inventory)  (psst check out [Overland](https://overland-game.com/))

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

