{pkgs ? import <nixpkgs> {}}: let
  cargotoml = builtins.fromTOML (builtins.readFile ../Cargo.toml);
in
  pkgs.rustPlatform.buildRustPackage {
    pname =
      cargotoml
      .package
      .name;
    version =
      cargotoml
      .package
      .version;
    cargoLock.lockFile = ../Cargo.lock;
    src = pkgs.lib.cleanSource ../.;
  }
