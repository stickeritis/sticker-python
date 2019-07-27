{ callPackage }:

callPackage (builtins.fetchTarball {
  url = "https://git.sr.ht/~danieldk/nix-packages/archive/25911247666aff6b5951b0665ce4e9c152d56d84.tar.gz";
  sha256 = "0vj4x0wmk5akvv2d6ibnalpsz3s06vmi1z9a8qwhla4f9jnfr52w";
}) {}
