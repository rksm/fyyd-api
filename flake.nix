{
  description = "fyyd-api - rust bindings to the fyyd-api";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  inputs.rust-overlay = {
    url = "github:oxalica/rust-overlay";
    inputs.nixpkgs.follows = "nixpkgs";
    inputs.flake-utils.follows = "flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem
    (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      stdenv =
        if pkgs.stdenv.isLinux
        then pkgs.stdenvAdapters.useMoldLinker pkgs.stdenv
        else pkgs.stdenv;
      overlays = [(import rust-overlay)];
      rustPkgs = import nixpkgs {
        inherit system overlays;
      };
      src = self;
      name = "fyyd";
      RUST_TOOLCHAIN = src + "/rust-toolchain.toml";
      rustToolchainTOML = rustPkgs.rust-bin.fromRustupToolchainFile RUST_TOOLCHAIN;
      rustToolchainDevTOML = rustToolchainTOML.override {
        extensions = ["rustfmt" "clippy" "rust-analysis" "rust-docs"];
        targets = [];
      };

      buildInputs = [
        pkgs.installShellFiles
        pkgs.sqlite
        pkgs.openssl
      ];
      nativeBuildInputs = [
        pkgs.pkg-config
      ];
      devInputs = [
        rustToolchainDevTOML
        pkgs.just

        pkgs.cargo-bloat
        pkgs.cargo-deny
        pkgs.cargo-diet
        pkgs.cargo-dist
        pkgs.cargo-flamegraph
        pkgs.cargo-modules
        pkgs.cargo-nextest
        pkgs.cargo-outdated
        pkgs.cargo-public-api
        pkgs.cargo-unused-features
        pkgs.cargo-watch

        # snapshot testing
        pkgs.cargo-insta

        pkgs.openssl # for `cargo xtask`

        pkgs.reuse
        pkgs.lychee

        pkgs.rust-analyzer

        (pkgs.symlinkJoin {
          name = "cargo-udeps-wrapped";
          paths = [pkgs.cargo-udeps];
          nativeBuildInputs = [pkgs.makeWrapper];
          postBuild = ''
            wrapProgram $out/bin/cargo-udeps \
              --prefix PATH : ${pkgs.lib.makeBinPath [
              (rustPkgs.rust-bin.selectLatestNightlyWith
                (toolchain: toolchain.default))
            ]}
          '';
        })
        (pkgs.symlinkJoin {
          name = "cargo-careful-wrapped";
          paths = [pkgs.cargo-careful];
          nativeBuildInputs = [pkgs.makeWrapper];
          postBuild = ''
            wrapProgram $out/bin/cargo-careful \
              --prefix PATH : ${pkgs.lib.makeBinPath [
              (rustPkgs.rust-bin.selectLatestNightlyWith
                (
                  toolchain:
                    toolchain
                    .default
                    .override {
                      extensions = ["rust-src"];
                    }
                ))
            ]}
          '';
        })
        #alternative linker
        pkgs.llvmPackages.bintools
        pkgs.mold
        pkgs.clang
      ];
      shellInputs = [
        pkgs.shellcheck
        pkgs.actionlint
      ];
      fmtInputs = [
        pkgs.alejandra
        pkgs.treefmt
        pkgs.typos
      ];
      editorConfigInputs = [
        pkgs.editorconfig-checker
      ];
      actionlintInputs = [
        pkgs.actionlint
      ];
    in {
      devShells = {
        default = (pkgs.mkShell.override {inherit stdenv;}) {
          buildInputs = shellInputs ++ fmtInputs ++ devInputs ++ buildInputs ++ nativeBuildInputs;
          inherit name;
          RUST_BACKTRACE = true;
        };
        editorConfigShell = pkgs.mkShell {
          buildInputs = editorConfigInputs;
        };
        actionlintShell = pkgs.mkShell {
          buildInputs = actionlintInputs;
        };
        fmtShell = pkgs.mkShell {
          buildInputs = fmtInputs;
        };
      };
      formatter = pkgs.alejandra;
    });
}
