{
  description = "random input generator for competitive programming";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
      toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      rustPlatform = pkgs.makeRustPlatform {
        cargo = toolchain;
        rustc = toolchain;
      };
      manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
      randGen = rustPlatform.buildRustPackage {
        pname = manifest.name;
        version = manifest.version;
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
      };
    in
    {
      checks.${system}.default = randGen;
      devShells.${system}.default = pkgs.mkShell {
        packages = [ toolchain ];
        shellHook = ''
          echo "rand-gen environment"
          echo "  rust: $(rustc --version)"
        '';
      };
    };
}
