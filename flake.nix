{
  description = "hb development environment";

  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/release-22.11";
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , fenix
    }:

    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs { inherit system; };
      rustToolchain = fenix.packages.${system}.stable.toolchain;
      rustPlatform = (pkgs.makeRustPlatform {
        cargo = rustToolchain;
        rustc = rustToolchain;
      });

      name = "hb";
      version = "0.3.0";
      deps = with pkgs; [
        rustToolchain
        openssl
        pkg-config
        cargo-deny
        cargo-edit
        cargo-make
        cargo-nextest
        cargo-watch
        rust-analyzer
      ];
      dev-deps = with pkgs; [
        cachix
        jq
        p7zip
      ];

      drv = rustPlatform.buildRustPackage {
        pname = "${name}";
        version = "${version}";
        src = builtins.path {
          path = ./.;
          name = "${name}";
        };
        cargoLock = {
          lockFile = ./Cargo.lock;
        };
      };
      app = flake-utils.lib.mkApp {
        inherit name drv;
      };
      shell = pkgs.mkShell {
        packages = deps ++ dev-deps;
      };
    in {
      # `nix build`
      packages = {
        "${name}" = drv;
        default = drv;
      };

      # `nix run`
      apps = {
        "${name}" = app;
        default = app;
      };

      # `nix develop`
      devShells = {
       "${name}" = shell;
        default = shell;
      };      
    });
}

