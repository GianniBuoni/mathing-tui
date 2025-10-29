{inputs, ...}: {
  imports = with inputs.rust-flake.flakeModules; [
    default
    nixpkgs
  ];

  perSystem = {
    self',
    pkgs,
    lib,
    config,
    ...
  }: {
    rust-project = {
      src = lib.cleanSourceWith {
        src = inputs.self;
        filter = path: type:
          (lib.hasInfix ".sqlx/" path)
          || (lib.hasSuffix ".sql" path)
          || (config.rust-project.crane-lib.filterCargoSources path type);
      };
      crates.mathing.crane = {
        args = {
          buildInputs = with pkgs; [openssl];
          nativeBuildInputs = with pkgs; [pkg-config];
        };
        extraBuildArgs.SQLX_OFFLINE = "true";
      };
    };
    packages.default = self'.packages.mathing;
  };
}
