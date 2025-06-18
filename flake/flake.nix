{

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      cargo_toml = (builtins.fromTOML (builtins.readFile ../Cargo.toml));
    in {

      defaultPackage.${system} = pkgs.rustPlatform.buildRustPackage {
        pname = cargo_toml.package.name;
        version = cargo_toml.package.version;
        cargoLock.lockFile = ../Cargo.lock;
        src = ../.;
      };

      devShells.${system}.default = with pkgs; mkShell {
        buildInputs = [ cargo rustc rustfmt rust-analyzer clippy ];
      };

    };

}
