{
  description = "Hyprland Desktop Shell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      ...
    }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};

      craneLib = (crane.mkLib nixpkgs.legacyPackages.${system});
      src =
        let
          root = ./.;
          fileset = pkgs.lib.fileset.unions [
            (craneLib.fileset.commonCargoSources ./.)
            (pkgs.lib.fileset.fileFilter (file: file.hasExt "scss") ./.)
          ];
        in
        pkgs.lib.fileset.toSource { inherit root fileset; };

      commonArgs = {
        inherit src;
        strictDeps = true;

        buildInputs = with pkgs; [
          pam
          dbus
          gtk4
          glib
          gtk4-layer-shell
        ];

        nativeBuildInputs = with pkgs; [
          pkg-config
          wrapGAppsHook4
        ];
      };

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      seashell = craneLib.buildPackage (
        commonArgs
        // {
          inherit cargoArtifacts;
        }
      );

      devShell = craneLib.devShell.override {
        mkShell = pkgs.mkShell.override {
          stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.stdenv;
        };
      };
    in
    {
      checks.${system} = {
        inherit seashell;

        clippy = craneLib.cargoClippy (
          commonArgs
          // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          }
        );

        fmt = craneLib.cargoFmt {
          inherit src;
        };
      };

      packages.${system}.default = seashell;

      devShells.${system}.default = devShell {
        checks = self.checks.${system};

        packages = with pkgs; [
          rust-analyzer
          zbus-xmlgen
        ];

        # Convinient logging for develpment
        RUST_BACKTRACE = 1;
        RUST_LOG = "info";
      };
    };
}
