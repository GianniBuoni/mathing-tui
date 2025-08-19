{
  description = "Mathing: Expense splitting in the terminal!";

  inputs = {
    devenv.url = "github:cachix/devenv";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {nixpkgs.follows = "nixpkgs";};
    };
  };

  outputs = {
    self,
    flake-utils,
    naersk,
    nixpkgs,
    devenv,
    ...
  } @ inputs:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = (import nixpkgs) {inherit system;};
      naersk' = pkgs.callPackage naersk {};
      pname = "mathing";
    in {
      defaultPackage = naersk'.buildPackage {
        inherit pname;
        src = ./.;
        cargoBuildOptions = defaults: defaults ++ ["--bin" "mathing"];
        singleStep = true;
      };
      packages.test = naersk'.buildPackage {
        inherit pname;
        src = ./.;
        mode = "test";
        release = false;
      };
      devShell = devenv.lib.mkShell {
        inherit inputs pkgs;
        modules = [./devenv.nix];
      };
    });
}
