{ callPackage, fetchFromGitHub }:

callPackage (fetchFromGitHub {
  owner = "stickeritis";
  repo = "nix-packages";
  rev = "9ca1cfb";
  sha256 = "1292bamr002spby7w0l6k2fx965qg0lzg7n2ny6nzw32k13gq3wq";
}) {}
