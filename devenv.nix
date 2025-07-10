{pkgs, ...}: {
  packages = with pkgs; [
    just
    openssl
    # sqlx is installed using cargo
  ];

  enterShell = ''export MATHING_CONFIG=$PWD/.config/mathing'';

  enterTest = ''
    cargo --version
    just --version
  '';

  languages.rust = {
    enable = true;
    channel = "stable";
  };

  dotenv.enable = true;
}
