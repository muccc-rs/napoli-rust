{
  description = "Enterprise grade food ordering system";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-utils.url = "github:numtide/flake-utils";
    flake-utils.inputs.nixpkgs.follows = "nixpkgs";

    rust-overlay.url = "github:oxalica/rust-overlay";

    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, flake-utils, naersk, nixpkgs, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [
          (import rust-overlay)
        ];
        pkgs = import nixpkgs {
          inherit system overlays;

        };

        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        dev-toolchain = toolchain.override {
          extensions = [ "rust-src" "rls" "rustfmt" ];
        };

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };
      in rec {

        defaultPackage = naersk'.buildPackage {
          src = ./.;
        };

        devShell = pkgs.mkShell rec {
          name = "napoli-rust";

          packages = with pkgs; [
            # Development Tools
            dev-toolchain
            pkgconf
            pkg-config
            protobuf
            mob
            # JS / CSS management
            nodejs
            nodePackages.npm
            nodePackages.tailwindcss
          ] ++ (
            # Apple libraries if necessary
            lib.optional stdenv.isDarwin [
              darwin.apple_sdk.frameworks.Security
            ]
          );
        };
      }
    );
}
