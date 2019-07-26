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
  libtensorflow_1_14_0 = (callPackage ./nix/danieldk.nix {}).libtensorflow_1_14_0;
  rustPlatform = callPackage ./nix/rust-nightly.nix {};
in rustPlatform.buildRustPackage rec {
  pname = "sticker";
  version = "0.1.0";

  src = nix-gitignore.gitignoreSource [ ".git/" "*.nix" "/nix" ] ./.;
  cargoSha256 = "1i1fwi744s2adp27vnc1h7nz3w8knyjchly55m599wkpqv81a1q9";

  nativeBuildInputs = [ pkgconfig pyo3-pack ];

  buildInputs = [ libtensorflow_1_14_0 openssl python3 ];

  installCheckInputs = [ python3Packages.pytest ];

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
