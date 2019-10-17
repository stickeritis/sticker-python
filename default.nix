{
  callPackage
, stdenv

, defaultCrateOverrides
, nix-gitignore

, curl
, darwin
, python3Packages
, pkgconfig

, releaseBuild ? true
}:

let
  libtensorflow_1_14_0 = (callPackage ./nix/sticker.nix {}).libtensorflow_1_14_0;
  rustNightly = callPackage ./nix/rust-nightly.nix {};
  cargo_nix = callPackage ./nix/Cargo.nix {};
in cargo_nix.rootCrate.build.override {
  rust = rustNightly;
  release = releaseBuild;

  buildInputs = stdenv.lib.optional stdenv.isDarwin darwin.Security;

  crateOverrides = defaultCrateOverrides // {
    sticker-python = attr: rec {
      pname = "sticker-python";
      name = "${pname}-${attr.version}";

      type = [ "cdylib" ];

      src = nix-gitignore.gitignoreSource [ ".git/" "*.nix" "/nix" ] ./.;

      installCheckInputs = [ python3Packages.pytest ];

      doInstallCheck = true;

      installPhase = let
        sitePackages = python3Packages.python.sitePackages;
        sharedLibrary = stdenv.hostPlatform.extensions.sharedLibrary;
      in ''
        runHook preInstall

        mkdir -p "$out/${sitePackages}"
        cp target/lib/libsticker_python*${sharedLibrary} \
          "$out/${sitePackages}/sticker.so"
        export PYTHONPATH="$out/${sitePackages}:$PYTHONPATH"

        runHook postInstall
      '';

      installCheckPhase = ''
        runHook preInstallCheck

        pytest

        runHook postInstallCheck
      '';

      meta = with stdenv.lib; {
        description = "Python module for the sticker sequence labeler";
        license = licenses.asl20;
        platforms = platforms.all;
      };
    };

    sticker-utils = attr: {
      # We only care for the library part of sticker-utils.
      crateBin = [];
    };

    pyo3 = attr: {
      buildInputs = [ python3Packages.python ];
    };

    tensorflow-sys = attrs: {
      nativeBuildInputs = [ pkgconfig ];

      buildInputs = [ libtensorflow_1_14_0 ] ++
        stdenv.lib.optional stdenv.isDarwin curl;
    };
  };
}
