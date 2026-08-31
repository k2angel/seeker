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

        inherit
          (craneLib.crateNameFromCargoToml {
            cargoToml = ./seeker/Cargo.toml;
          })
          pname
          version
          ;

        src = lib.fileset.toSource {
          root = unfilteredRoot;
          fileset = lib.fileset.unions [
            (craneLib.fileset.commonCargoSources unfilteredRoot)
            (lib.fileset.fileFilter (file: file.hasExt "sql") unfilteredRoot)
            (lib.fileset.maybeMissing ./seeker_core/tests/data)
          ];
        };

        commonArgs = {
          inherit pname version src;
          strictDeps = true;
          buildInputs = with pkgs; [
            sqlite
            openssl
          ];
          nativeBuildInputs = [ pkgs.pkg-config ];

          cargoBuildExtraArgs = "-p seeker";
          cargoTestExtraArgs = "-p seeker_core";
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
