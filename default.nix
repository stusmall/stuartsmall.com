{ nixpkgs ? import <nixpkgs> { } }:

let
  src = nixpkgs.fetchFromGitHub {
    owner = "cooklang";
    repo = "cookcli";
    rev = "v0.6.0";
    hash = "sha256-d+jPaPYvtFaiWaNyyOpazmhR3LSe5DfKjdM4dkdaQHw=";

  };
in
let
  ui = nixpkgs.buildNpmPackage {
    name = "ui";
    src = "${src}/ui";
    npmDepsHash = "sha256-OTSes4yratfPJippeKfXQSyRmdYw0MbGHU42LzVugl4=";
    makeCacheWritable = true;
    npmFlags = [ "--legacy-peer-deps" ];
    installPhase = ''
      runHook preInstall
      mv public/ $out
      runHook postInstall
    '';
  };
  source = src;
in
let
  cooklang = nixpkgs.rustPlatform.buildRustPackage {
    pname = "cookcli";
    version = "0.6.0";
    src = source;
    cargoHash = "sha256-DYyzo3M/JCl0pXr53KnAUO8UkAJltSXnEW90KPA1aQY=";
    nativeBuildInputs = [ nixpkgs.pkg-config nixpkgs.openssl ];
    buildInputs = [ nixpkgs.openssl ];
    postPatch = ''
      cp -r ${ui}/* "ui/public"
    '';
    OPENSSL_NO_VENDOR = 1;
  };
in
nixpkgs.mkShell {
  buildInputs = [ cooklang nixpkgs.zola ];
}


