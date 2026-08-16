**This is an archived version of [the original repository](https://tangled.org/hannobraun.com/monobloc), as I may delete that later.**

# Monobloc

Monobloc is a programming language with the goal of making programming more fun and productive, by providing tighter feedback loops and better feedback than most languages usually do.

## Status

**This is an aborted attempt to clean up and rewrite parts of the Monobloc prototype from my private repository.** I initially planned to publish Monobloc incrementally, alongside my prototyping work, but found that this was putting undue pressure on the design process.

**This repository is now retired.** I may decide to reuse it, once I'm ready for a public release again, or maybe I'll delete and replace it.

[This Bluesky thread](https://bsky.app/profile/hannobraun.com/post/3mt4lbeoht22r) has some more context.

## Development

To work on this project, you need to install some system dependencies, as specified in [`shell.nix`]. How to do that depends on your local environment. If you're running [NixOS] with [nix-direnv], they will be installed automatically as you enter the directory (after you give permission).

Other than that, this is a pretty standard Rust project, packaged as a library. Run a regular build with `cargo clippy` and the tests with `cargo test`.

You can run the full CI build locally with `cargo run`. This must pass without errors or warnings, ideally for every single commit, for a pull request to get merged.

[`shell.nix`]: shell.nix
[NixOS]: https://nixos.org/
[nix-direnv]: https://github.com/nix-community/nix-direnv

## License

This repository is open source, licensed under the terms of the [Zero Clause BSD License] (0BSD, for short). This basically means you can do anything with the code here, without any restrictions, but you can't hold the authors liable for problems.

See [LICENSE.md] for full details.

[Zero Clause BSD License]: https://opensource.org/licenses/0BSD
[LICENSE.md]: LICENSE.md
