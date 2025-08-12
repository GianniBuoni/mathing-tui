{
  description = "Mathing: Expense splitting in the terminal!";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
  in {
    packages."x86_64-linux".default = pkgs.rustPlatform.buildRustPackage {
      name = "mathing";
      src = ./.;
      buildInputs = [pkgs.openssl];
      nativeBuildInputs = [pkgs.pkg-config];
      cargoLock.lockFile = ./Cargo.lock;
      cargoBuildFlags = "--bin mathing";
    };
  };
}
