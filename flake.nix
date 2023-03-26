{
  description = "hb development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-22.11";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , rust-overlay
    }:

    flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [
        (import rust-overlay)
        (self: super: {
          rustToolchain =
            let
              rust = super.rust-bin;
            in
            if builtins.pathExists ./rust-toolchain.toml then
              rust.fromRustupToolchainFile ./rust-toolchain.toml
            else if builtins.pathExists ./rust-toolchain then
              rust.fromRustupToolchainFile ./rust-toolchain
            else
              rust.stable.latest.default;
        })
      ];

      pkgs = import nixpkgs { inherit system overlays; };
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

      drv = pkgs.rustPlatform.buildRustPackage {
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
        packages = deps;
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

