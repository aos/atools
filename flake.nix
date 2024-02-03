{
  description = "Rusty tools";

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
      perSystem = { pkgs, system, ... }:
        let
          _module.args.pkgs = import nixpkgs {
            inherit system;
            overlays = [ fenix.overlays.default ];
          };
          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        in
        {
          packages = {
            default = pkgs.rustPlatform.buildRustPackage rec {
              name = cargoToml.package.name;
              version = cargoToml.package.version;
              src = ./.;
              cargoLock.lockFile = ./Cargo.lock;

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
          };
          devShells.default = with pkgs; mkShell {
            nativeBuildInputs = [
              fenix.packages.${system}.stable.toolchain
              rust-analyzer
              pkg-config
            ];

            RUST_BACKTRACE = 1;
          };
        };
    };
}
