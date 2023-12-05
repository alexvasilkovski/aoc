{ pkgs ? import <nixpkgs> {} }:
let
  rust-overlay = (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"));
  pkgs = (import <nixpkgs> {
    overlays = [ rust-overlay ];
  });
in
  pkgs.mkShell rec {

    buildInputs = with pkgs; [
      rustc
      clang
      llvmPackages_16.bintools
      rustup
      bashInteractive

      (pkgs.rust-bin.stable.latest.rust.override {
      extensions = ["rust-src"];
    })];

    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

    RUSTC_VERSION = "stable";

    LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];

    shellHook = ''
      export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
      export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
      '';

    RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') [
    ]);

    BINDGEN_EXTRA_CLANG_ARGS = 
    (builtins.map (a: ''-I"${a}/include"'') [
      pkgs.glibc.dev 
    ])
    
    ++ [
      ''-I"${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include"''
      ''-I"${pkgs.glib.dev}/include/glib-2.0"''
      ''-I${pkgs.glib.out}/lib/glib-2.0/include/''
    ];
  }