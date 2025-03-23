let
  pkgs = import (
    builtins.fetchGit {
      name = "nixpkgs-jan-25-2025";
      url = "https://github.com/nixos/nixpkgs/";
      ref = "refs/heads/master";
      rev = "aeba1dd05ab1f729eeac31677abfd39092fd1ee0";
    }) {};
in
pkgs.mkShell {
  buildInputs = [
    pkgs.cargo
    pkgs.rustc
    pkgs.rustfmt
    pkgs.zola
  ];
}


