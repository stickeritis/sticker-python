{ callPackage }:

callPackage (builtins.fetchTarball {
  url = "https://git.sr.ht/~danieldk/nix-packages/archive/d84cba3b742f94d587191d8c794a7cfc762c1c60.tar.gz";
  sha256 = "1drsg97mfjwmbph8q7f4hpgiyjxvy9ifn83mbpxfkgsa0i7qczdp";
}) {}
