{ fetchFromGitHub, callPackage, makeRustPlatform }:

let
  gitignore = fetchFromGitHub {
    owner = "hercules-ci";
    repo = "gitignore";
    rev = "ec5dd0536a5e4c3a99c797b86180f7261197c124";
    sha256 = "0k2r8y21rn4kr5dmddd3906x0733fs3bb8hzfpabkdav3wcy3klv";
  };
in (callPackage "${gitignore.out}/default.nix" {}).gitignoreSource
