{
  description = "A basic flake with a shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    systems.url = "github:nix-systems/default";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.systems.follows = "systems";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        lib = pkgs.lib;
        craneLib = crane.mkLib pkgs;
        unfilteredRoot = ./.;

        src = lib.fileset.toSource {
          root = unfilteredRoot;
          fileset = lib.fileset.unions [
            (craneLib.fileset.commonCargoSources unfilteredRoot)
            (lib.fileset.fileFilter (file: file.hasExt "sql") unfilteredRoot)
            (lib.fileset.maybeMissing ./tests/data)
          ];
        };

        commonArgs = {
          inherit src;
          strictDeps = true;
          buildInputs = with pkgs; [
            pkgs.sqlite
            pkgs.openssl
          ];
          nativeBuildInputs = [ pkgs.pkg-config ];
        };

        cargoArtifacts = craneLib.buildDepsOnly (
          commonArgs
          // {
            pname = "mycrate-deps";
          }
        );

        myCrateClippy = craneLib.cargoClippy (
          commonArgs
          // {
            inherit cargoArtifacts;
          }
        );

        myCrate = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
          }
        );
      in
      {
        checks = { inherit myCrate myCrateClippy; };
        packages.default = myCrate;

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            cargo
            clippy
            sqlite
            openssl
          ];
        };
      }
    );
}
