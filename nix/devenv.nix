{inputs, ...}: {
  imports = [inputs.devenv.flakeModule];

  perSystem = {pkgs, ...}: {
    devenv.shells.default = {
      # local devenv vars
      env.SQLX_OFFLINE = "true";
      env.DATABASE_URL = "sqlite://./.config/mathing/data.db";

      packages = with pkgs; [
        just
        openssl
        sqlx-cli
      ];

      enterTest = ''
        cargo --version
        just --version
        sqlx --version
      '';
      languages.rust = {
        enable = true;
        channel = "stable";
      };
    };
  };
}
