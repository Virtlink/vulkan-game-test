# Vulkan Game Test

## Development

1.  Ensure you have [Rust][1] installed.

2.  Check out this repository and its submodules.

        git clone https://github.com/Virtlink/vulkan-game-test.git
        git submodule update --init --recursive

3.  Override the Rust toolchain in the root directory to use _nightly_:

        rustup override set nightly

4.  Build and run the project:

        cargo run



[1]: https://rustup.rs/