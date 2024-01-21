{ nixpkgs ? import <nixpkgs> { } }:


nixpkgs.mkShell {
  buildInputs = [
    nixpkgs.cargo 
    nixpkgs.rustc 
    nixpkgs.rustfmt 
    nixpkgs.zola 
  ];
}


