{
  callPackage
, lib

, nix-gitignore
, pkgconfig
, pyo3-pack
, python3Packages

, python3
, libtensorflow
, openssl

, releaseBuild ? true
}:

let
  rustPlatform = callPackage ./nix/rust-nightly.nix {};
  gitIgnore = callPackage ./nix/gitignore.nix {};
in rustPlatform.buildRustPackage rec {
  pname = "sticker";
  version = "0.1.0";

  src = nix-gitignore.gitignoreSource [".git/"] ./.;
  cargoSha256 = "1kiqbi64s8bzlyaiq2mk2642zsff7h0cr1yjf5drhizmi1fdvw5y";

  nativeBuildInputs = [ pkgconfig pyo3-pack ];

  buildInputs = [ libtensorflow openssl python3 ];

  installCheckInputs = [ python3Packages.pytest];

  doInstallCheck = true;

  buildPhase = let
    releaseArg = lib.optionalString releaseBuild "--release";
  in ''
    pyo3-pack build ${releaseArg} --manylinux off
  '';

  installPhase = ''
    mkdir -p "$out/${python3.sitePackages}"
    export PYTHONPATH="$out/${python3.sitePackages}:$PYTHONPATH"
    ${python3.pythonForBuild.pkgs.bootstrapped-pip}/bin/pip install \
        target/wheels/*.whl --no-index --prefix=$out --no-cache --build tmpbuild
  '';

  checkPhase = ''
    cargo fmt --all -- --check
    cargo clippy
  '';

  installCheckPhase = ''
    pytest
  '';
}
