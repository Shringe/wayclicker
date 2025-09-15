{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-25.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        name = cargoToml.package.name;
        version = cargoToml.package.version;
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          inherit name version;

          # Build-time dependencies
          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          # Runtime dependencies
          buildInputs = with pkgs; [
            libudev-zero
          ];

          cargoLock.lockFile = ./Cargo.lock;
          src = self;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            git
            rustc
            rustfmt
            cargo
            rust-analyzer

            libudev-zero
            pkg-config
          ];
        };
      }
    );
}
