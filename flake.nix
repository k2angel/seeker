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
          ];
        };

        testSrc = lib.fileset.toSource {
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
          buildInputs = with pkgs; [ sqlite ];
          nativeBuildInputs = with pkgs; [ pkg-config ];
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        myApp = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
            doCheck = false;
          }
        );

        myAppTest = craneLib.cargoTest (
          commonArgs
          // {
            inherit cargoArtifacts;
            src = testSrc;
          }
        );
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            cargo
            sqlite
          ];
        };

        packages.default = myApp;
        checks = { inherit myAppTest; };
      }
    );
}
