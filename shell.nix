{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  packages = with pkgs; [
    # This project is written in Rust, and Rustup is the best way to get a Rust
    # toolchain.
    rustup

    # Rust uses `cc` as a linker, which this package provides.
    clang

    # Used for mutation testing. https://mutants.rs/
    cargo-mutants
  ];
}
