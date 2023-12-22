{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    devenv.url = "github:cachix/devenv";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    devenv,
    ...
  } @ inputs: let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
  in {
    devShell.x86_64-linux = devenv.lib.mkShell {
      inherit inputs pkgs;
      modules = [
        ({
          pkgs,
          config,
          ...
        }: {
          # This is your devenv configuration
          packages = [];

          languages.rust = {
            enable = true;
            channel = "nightly";
          };

          pre-commit.hooks = {
            alejandra.enable = true;
            commitizen.enable = true;
            clippy.enable = true;
            cargo-check.enable = true;
            rustfmt.enable = true;
          };
        })
      ];
    };
  };
}
