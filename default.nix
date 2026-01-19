let
  rust_overlay = import (
    builtins.fetchGit {
      name = "rust-overlay-dec-30-2025";
      url = "https://github.com/oxalica/rust-overlay/";
      ref = "refs/heads/master";
      rev = "943d2d0bf1b86267287aff826ebd1138d83113b7";
    }
  );
  pkgs = import (builtins.fetchGit {
    name = "nixpkgs-jan-18-2026";
    url = "https://github.com/nixos/nixpkgs/";
    ref = "refs/heads/nixos-25.11";
    rev = "72ac591e737060deab2b86d6952babd1f896d7c5";
  }) { overlays = [ rust_overlay ]; };
  rust_toolchain = pkgs.rust-bin.fromRustupToolchainFile ./build-md/rust-toolchain.toml;
in
pkgs.mkShell {
  buildInputs = [
    rust_toolchain
    pkgs.libwebp
    pkgs.zola
  ];
}
