{
  description = "Rust project with required dependencies and automatic cargo run";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = [
            pkgs.rustup
            pkgs.nodejs
            pkgs.cargo-tauri
            pkgs.pkg-config
            pkgs.cargo-tauri
            pkgs.webkitgtk
            pkgs.libsoup
            pkgs.git
          ];

          shellHook = ''
            rustup toolchain install stable
            rustup default stable
            rustup target add wasm32-unknown-unknown
            cargo install trunk
            cargo install cargo-leptos
            cargo tauri dev
          '';
        };
      });
}
