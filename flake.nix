{
  description = "Mathing: Expense splitting in the terminal!";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = {
    self,
    flake-utils,
    naersk,
    nixpkgs,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = (import nixpkgs) {inherit system;};
      naersk' = pkgs.callPackage naersk {};
      pname = "mathing";
    in {
      defaultPackage = naersk'.buildPackage {
        inherit pname;
        src = ./.;
        cargoBuildOptions = defaults: defaults ++ ["--bin" "mathing"];
      };
    });
}
