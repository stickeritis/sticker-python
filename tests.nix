{
  callPackage
, runCommand

, python3Packages

, releaseBuild ? true
}:

let
  sticker-python = callPackage ./default.nix { inherit releaseBuild; };
  models = (callPackage ./nix/sticker.nix {}).models;
in runCommand "test-tagger" {
  nativeBuildInputs = [ python3Packages.pytest sticker-python ];
} ''
  pytest -p no:cacheprovider \
         --tagger_model=${models.de-pos-ud.model}/share/sticker/models/de-pos-ud/sticker.conf \
         --topo_model=${models.de-topo-ud-small.model}/share/sticker/models/de-topo-ud-small/sticker.conf \
    ${sticker-python.src}
  touch $out
''
