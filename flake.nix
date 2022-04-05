{
  inputs = {
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk.url = "github:nmattia/naersk";
  };
  outputs = { self, flake-utils, fenix, nixpkgs, naersk, flake-compat, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        # Add rust nightly to pkgs
        pkgs = nixpkgs.legacyPackages.${system} // { inherit (fenix.packages.${system}.latest) cargo rustc rust-src clippy-preview rustfmt-preview; };

        naersk-lib = (naersk.lib."${system}".override {
          cargo = pkgs.cargo;
          rustc = pkgs.rustc;
        });

        todo =
          naersk-lib.buildPackage {
            pname = "todo";
            root = ./.;
          };

      in rec {
        packages.todo = todo;

        defaultPackage = self.packages.${system}.todo;

        apps.todo = flake-utils.lib.mkApp { drv = packages.todo; };
        defaultApp = apps.todo;

        devShell = import ./shell.nix { inherit pkgs; };
      });
    }
