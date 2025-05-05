{pkgs, ...}: {
  packages = with pkgs; [
    just
    openssl
    # sqlx is installed using cargo
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

  dotenv.enable = true;
}
