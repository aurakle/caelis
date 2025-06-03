let
  nixpkgsVer = "10d7f8d34e5eb9c0f9a0485186c1ca691d2c5922";
  pkgs = import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/${nixpkgsVer}.tar.gz") { config = {}; overlays = []; };
  libs = with pkgs; [
  ];
in pkgs.mkShell {
  name = "caelis";

  buildInputs = libs ++ (with pkgs; [
    cargo
    rustc
    gcc
    rustfmt
    pkg-config
  ]);

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  RUST_BACKTRACE = 1;
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libs;
  LLVM_SYS_181_PREFIX = "${pkgs.llvmPackages_18.libllvm.dev}";
}
