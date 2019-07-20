with import <nixpkgs> {};

let
  danieldk = pkgs.callPackage (builtins.fetchTarball {
    url = "https://git.sr.ht/~danieldk/nix-packages/archive/709c93a84504d558613bfc2538297ef2c532b890.tar.gz";
    sha256 = "0jspqxz8yzghn4j3awiqz3f76my8slk3s5ckk3gfzvhq1p0wzp5m";
  }) {};
  mozilla = fetchFromGitHub {
    owner = "mozilla";
    repo = "nixpkgs-mozilla";
    rev = "9f35c4b09fd44a77227e79ff0c1b4b6a69dff533";
    sha256 = "18h0nvh55b5an4gmlgfbvwbyqj91bklf1zymis6lbdh75571qaz0";
  };
  rustNightly =
    with import "${mozillaOverlay.out}/rust-overlay.nix" pkgs pkgs;
    (rustChannelOf { date = "2019-02-07"; channel = "nightly"; }).rust;
in stdenv.mkDerivation rec {
  name = "sticker-env";
  env = buildEnv { name = name; paths = buildInputs; };

  nativeBuildInputs = [
    pkgconfig
    latest.rustChannels.nightly.rust
    pyo3-pack
    python3
    python3Packages.pytest
  ];

  buildInputs = [
    curl
    danieldk.libtensorflow_1_14_0
    openssl
  ] ++ lib.optional stdenv.isDarwin darwin.apple_sdk.frameworks.Security;
}
