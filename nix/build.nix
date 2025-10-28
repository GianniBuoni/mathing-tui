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
        # add `.sqlx/*` to project source files
          (lib.hasInfix ".sqlx/" path)
          # add `sql` files to project src
          || (lib.hasSuffix ".sql" path)
          # default crane project src filter
          || (config.rust-project.crane-lib.filterCargoSources path type);
      };
      crates.mathing.crane = {
        args = {
          buildInputs = with pkgs; [
            openssl
          ];
          nativeBuildInputs = with pkgs; [
            sqlx-cli
            pkg-config
          ];
        };
        extraBuildArgs.SQLX_OFFLINE = "true";
      };
    };
    packages.default = self'.packages.mathing;
  };
}
