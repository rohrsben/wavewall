{
    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
        rust-overlay.url = "github:oxalica/rust-overlay";
        flake-utils.url = "github:numtide/flake-utils";
    };

    outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
        flake-utils.lib.eachDefaultSystem (system:
        let
            overlays = [ (import rust-overlay) ];
            pkgs = import nixpkgs {
                inherit system overlays;
            };
        in {
            packages."x86_64-linux" = 
            let
                manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
            in
                pkgs.rustPlatform.buildRustPackage {
                    pname = "wavewall";
                    version = "0.1.0";
                    cargoLock.lockFile = ./Cargo.lock;
                    src = pkgs.lib.cleanSource ./.;
            };
            devShells.default = pkgs.mkShell {
                buildInputs = with pkgs; [
                    rust-bin.beta.latest.default
                    rust-analyzer
                    nodePackages.pnpm
                    nodejs
                ];
            };
        }
    );
}
