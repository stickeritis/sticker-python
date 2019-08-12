{ callPackage }:

callPackage (builtins.fetchTarball {
  url = "https://git.sr.ht/~danieldk/nix-packages/archive/c4a277bc5afaecbf982c3fa19cd7bac68de1826f.tar.gz";
  sha256 = "1n070qjp31fc94z5s1lglihlk7pnq14xdrvizjah9mfrwgaaa0p0";
}) {}
