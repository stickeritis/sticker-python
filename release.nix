let
  pkgs = import <nixpkgs> { config.allowUnfree = true; };
in {
  build = pkgs.callPackage ./default.nix {};
  testModel = pkgs.callPackage ./tests.nix {};
}
