{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  packages = with pkgs; [
    just
    qemu
  ];
  nativeBuildInputs = with pkgs; [
    OVMF
  ];

  shellHook = ''
    export OVMF_PATH="${pkgs.OVMF.fd}/FV"
  '';
}
