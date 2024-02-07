{
  description = "atools";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ flake-parts, nixpkgs, fenix, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];
      perSystem = { pkgs, system, self', ... }:
        let
          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          rustToolchain = fenix.packages.${system}.stable.toolchain;
        in
        {
          packages.atools = (pkgs.makeRustPlatform {
            cargo = rustToolchain;
            rustc = rustToolchain;
          }).buildRustPackage rec {
            name = cargoToml.package.name;
            version = cargoToml.package.version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;

            # Hard links the binaries to the main binary
            postInstall = ''
              ${builtins.concatStringsSep "\n"
                (builtins.map
                  (x: "ln $out/bin/${name} $out/bin/${pkgs.lib.removeSuffix ".rs" x}")
                  (builtins.filter
                    (n: n != "lib.rs" && n != "main.rs")
                    (builtins.attrNames
                      (builtins.readDir ./src))))}
            '';
          };
          packages.default = self'.packages.atools;
          devShells.default = self'.packages.default.overrideAttrs (super: {
            nativeBuildInputs = with pkgs;
              super.nativeBuildInputs
              ++ [
                fenix.packages.${system}.stable.rust-analyzer
              ];
            RUST_SRC_PATH = rustToolchain;
            RUST_BACKTRACE = 1;
          });
        };
    };
}
