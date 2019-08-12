{
  callPackage
, runCommand

, python3Packages

, releaseBuild ? true
}:

let
  sticker-python = callPackage ./default.nix { inherit releaseBuild; };
  de-pos-ud = (callPackage ./nix/danieldk.nix {}).stickerModels.de-pos-ud.model;
in runCommand "test-tagger" {
  nativeBuildInputs = [ python3Packages.pytest sticker-python ];
} ''
  pytest --tagger_model ${de-pos-ud}/share/sticker/models/de-pos-ud/sticker.conf ${sticker-python.src}
  touch $out
''
