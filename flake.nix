{
    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
        rust-overlay.url = "github:oxalica/rust-overlay";
    };

    outputs = { self, ... }@inputs:
        let
            overlays = [ (import inputs.rust-overlay) ];
            system = "x86_64-linux";
            pkgs = import inputs.nixpkgs {
                inherit system overlays;
            };
            manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
        in {
            packages.x86_64-linux = rec {
                wavewall = pkgs.rustPlatform.buildRustPackage {
                    pname = manifest.name;
                    version = manifest.version;
                    cargoLock.lockFile = ./Cargo.lock;
                    src = pkgs.lib.fileset.toSource {
                        root = ./.;
                        fileset = pkgs.lib.fileset.unions [
                            ./src
                            ./Cargo.lock
                            ./Cargo.toml
                        ];
                    };

                    strictDeps = true;
                };

                default = wavewall;
            };

            devShells.x86_64-linux.default = pkgs.mkShell {
                buildInputs = with pkgs; [
                    (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default))
                    rust-analyzer
                    nodePackages.pnpm
                    nodejs
                ];
            };
        };
}
