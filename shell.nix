{ pkgs ? import <nixpkgs> { } }:

with pkgs; mkShell {
  buildInputs = [
    espup
    cargo-espflash
  ];
}