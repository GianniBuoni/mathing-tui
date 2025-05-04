{pkgs, ...}: {
  packages = with pkgs; [
    just
    openssl
  ];

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
