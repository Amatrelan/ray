{
  description = "Simple backlight management cli tool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

    pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
    pre-commit-hooks.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    pre-commit-hooks,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        toolchain = pkgs.rust-bin.stable.latest.default;

        nativeBuildInputs = with pkgs; [
          rust-analyzer
          toolchain
        ];

        buildInputs = with pkgs; [];
      in {
        checks = {
          pre-commit-check = pre-commit-hooks.lib.${system}.run {
            src = ./.;
            hooks = {
              # Nix
              alejandra.enable = true;
              statix.enable = true;

              # Rust
              rustfmt.enable = true;
              # TODO: This uses different rustc than what it's compiled so it donesn't work.
              # and I don't have energy to check right now how to fix this.
              # clippy = {
              #   enable = true;
              # };
              cargo-check.enable = true;

              # Random
              typos.enable = true;
              commitizen.enable = true;
            };

            settings = {
              clippy.offline = false;
            };
          };
        };

        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs;
          inherit (self.checks.${system}.pre-commit-check) shellHook;
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          name = "ray";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          inherit buildInputs;
        };
      }
    );
}
