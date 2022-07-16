{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [ cargo rustc gcc ];
  buildInputs = with pkgs; [ rustfmt rust-analyzer ];
}

