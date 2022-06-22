with import <nixpkgs> {};
let
    # Pinned nixpkgs, deterministic. Last updated: 2/12/21.
    # pkgs = import (fetchTarball("https://github.com/NixOS/nixpkgs/archive/a58a0b5098f0c2a389ee70eb69422a052982d990.tar.gz")) {};

    # Rolling updates, not deterministic.
    pkgs = import (fetchTarball("channel:nixpkgs-unstable")) {};
    in pkgs.mkShell {
        buildInputs = [
            pkgs.cargo
            # pkgs.cargo-watch
            pkgs.clippy
            pkgs.git
            pkgs.just
            # pkgs.ngrok # TODO: Figure out this unfree bullshit
            pkgs.pgcli
            pkgs.rust-analyzer
            pkgs.rustc
            pkgs.rustfmt
            pkgs.shellcheck
            pkgs.sqlfluff
            pkgs.sqlx-cli
            darwin.apple_sdk.frameworks.CoreServices #stdenv.isDarwin only
            darwin.apple_sdk.frameworks.Security #stdenv.isDarwin only
            darwin.apple_sdk.frameworks.SystemConfiguration #stdenv.isDarwin only
            libiconv #stdenv.isDarwin only
        ];

        # Certain Rust tools won't work without this
        # This can also be fixed by using oxalica/rust-overlay and specifying the rust-src extension
        # See https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3?u=samuela. for more details.
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    }
