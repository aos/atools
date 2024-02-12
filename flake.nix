{
  description = "atools";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ self, nixpkgs, flake-utils, fenix, ... }:
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        rustToolchain = fenix.packages.${system}.stable.toolchain;
      in {
        packages = rec {
          ${cargoToml.package.name} = (pkgs.makeRustPlatform {
            cargo = rustToolchain;
            rustc = rustToolchain;
          }).buildRustPackage rec {
            name = cargoToml.package.name;
            version = cargoToml.package.version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;

            # Hard links the binaries to the main binary
            postInstall = ''
              ${builtins.concatStringsSep "\n" (builtins.map (x:
                "ln $out/bin/${name} $out/bin/${pkgs.lib.removeSuffix ".rs" x}")
                (builtins.filter (n: n != "lib.rs" && n != "main.rs")
                  (builtins.attrNames (builtins.readDir ./src))))}
            '';
          };
          default = ${cargoToml.package.name};
        };
        devShells.default = self.packages.${system}.default.overrideAttrs
          (super: {
            nativeBuildInputs = with pkgs;
              super.nativeBuildInputs
              ++ [ fenix.packages.${system}.stable.rust-analyzer ];
            RUST_SRC_PATH = rustToolchain;
            RUST_BACKTRACE = 1;
          });
      });
}
